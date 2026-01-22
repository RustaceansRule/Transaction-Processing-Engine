use crate::{
    common::{AccountsHash, InputData, TransactionHash},
    errors::AccountingError,
    models::{account::Account, transaction::Transaction},
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionKind {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

pub async fn run_engine(mut rx: tokio::sync::mpsc::Receiver<InputData>) -> AccountsHash {
    let mut accounts: AccountsHash = HashMap::with_capacity(10);
    let mut transactions: TransactionHash = HashMap::with_capacity(1_000);

    while let Some(d) = rx.recv().await {
        if let Err(_err) = process_record(&d, &mut accounts, &mut transactions) {
            // Action error here
        }
    }

    accounts
}

fn process_record(
    record: &InputData,
    accounts: &mut AccountsHash,
    transactions: &mut TransactionHash,
) -> Result<(), AccountingError> {
    let account = accounts.entry(record.client).or_insert(Account::default());

    // return without processing anything as account is locked
    if account.locked {
        return Err(AccountingError::AccountLocked);
    }

    match record.kind {
        TransactionKind::Deposit => {
            let amount = record.amount();
            account.deposit(amount);
            transactions.insert(record.tx, Transaction::new(record.client, amount, false));
        }
        TransactionKind::Withdrawal => {
            let amount = record.amount();
            if let Ok(()) = account.withdraw(amount) {
                transactions.insert(record.tx, Transaction::new(record.client, amount, false));
            }
        }
        TransactionKind::Dispute => {
            if let Some(transaction) = transactions.get_mut(&record.tx) {
                if !transaction.disputed() && transaction.client == record.client {
                    account.dispute(transaction.amount);
                    transaction.set_disputed();
                }
            }
        }
        TransactionKind::Resolve => {
            if let Some(transaction) = transactions.get_mut(&record.tx) {
                if transaction.disputed() && transaction.client == record.client {
                    account.resolve(transaction.amount);
                    transaction.resolved();
                }
            }
        }
        TransactionKind::Chargeback => {
            if let Some(transaction) = transactions.get_mut(&record.tx) {
                if transaction.disputed() && transaction.client == record.client {
                    account.chargeback(transaction.amount);
                    transaction.resolved();
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::{Decimal, prelude::FromPrimitive};
    use rust_decimal_macros::dec;

    fn deposit(c: u16, tx: u32, amt: i64) -> InputData {
        InputData::new(TransactionKind::Deposit, c, tx, Decimal::from_i64(amt))
    }

    fn dispute(c: u16, tx: u32) -> InputData {
        InputData::new(TransactionKind::Dispute, c, tx, Decimal::from_i64(0))
    }

    fn chargeback(c: u16, tx: u32) -> InputData {
        InputData::new(TransactionKind::Chargeback, c, tx, Decimal::from_i64(0))
    }

    #[test]
    fn deposit_and_chargeback_locks_account() {
        let mut accounts = HashMap::new();
        let mut txs = HashMap::new();

        process_record(&deposit(1, 1, 10), &mut accounts, &mut txs).unwrap();
        process_record(&dispute(1, 1), &mut accounts, &mut txs).unwrap();
        process_record(&chargeback(1, 1), &mut accounts, &mut txs).unwrap();

        let acc = &accounts[&1];
        assert!(acc.locked);
        assert_eq!(acc.total, dec!(0));
    }
}
