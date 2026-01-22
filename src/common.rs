use crate::{
    engine::TransactionKind,
    models::{account::Account, transaction::Transaction},
};
use rust_decimal::Decimal;
use serde::Deserialize;
use std::collections::HashMap;

pub type ClientId = u16;
pub type TransactionId = u32;
pub type AccountsHash = HashMap<ClientId, Account>;
pub type TransactionHash = HashMap<TransactionId, Transaction>;

#[derive(Deserialize)]
pub struct InputData {
    #[serde(rename = "type")]
    pub kind: TransactionKind,
    pub client: u16,
    pub tx: u32,
    #[serde(default)]
    amount: Option<Decimal>,
}

impl InputData {
    pub fn new(kind: TransactionKind, client: u16, tx: u32, amount: Option<Decimal>) -> Self {
        Self {
            kind,
            client,
            tx,
            amount,
        }
    }

    pub fn amount(&self) -> Decimal {
        self.amount.unwrap_or_default()
    }
}

#[allow(dead_code)] // allow unused as only call in tests
impl InputData {
    pub fn deposit(client: u16, tx: u32, amount: Decimal) -> InputData {
        InputData {
            kind: TransactionKind::Deposit,
            client,
            tx,
            amount: Some(amount),
        }
    }

    pub fn withdrawal(client: u16, tx: u32, amount: Decimal) -> InputData {
        InputData {
            kind: TransactionKind::Withdrawal,
            client,
            tx,
            amount: Some(amount),
        }
    }

    pub fn dispute(client: u16, tx: u32) -> InputData {
        InputData {
            kind: TransactionKind::Dispute,
            client,
            tx,
            amount: None,
        }
    }

    pub fn resolve(client: u16, tx: u32) -> InputData {
        InputData {
            kind: TransactionKind::Resolve,
            client,
            tx,
            amount: None,
        }
    }

    pub fn chargeback(client: u16, tx: u32) -> InputData {
        InputData {
            kind: TransactionKind::Chargeback,
            client,
            tx,
            amount: None,
        }
    }
}
