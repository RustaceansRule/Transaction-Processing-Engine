use crate::common::ClientId;
use rust_decimal::Decimal;

#[derive(Debug)]
pub struct Transaction {
    pub client: ClientId,
    pub amount: Decimal,
    disputed: bool,
}

impl Transaction {
    pub fn new(client: ClientId, amount: Decimal, disputed: bool) -> Transaction {
        Transaction {
            client,
            amount,
            disputed,
        }
    }

    pub fn disputed(&self) -> bool {
        self.disputed
    }
    pub fn set_disputed(&mut self) {
        self.disputed = true
    }

    pub fn resolved(&mut self) {
        self.disputed = false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn new_transaction_not_disputed() {
        let tx = Transaction::new(1, dec!(10), false);
        assert!(!tx.disputed());
    }

    #[test]
    fn set_disputed_marks_transaction() {
        let mut tx = Transaction::new(1, dec!(10), false);
        tx.set_disputed();
        assert!(tx.disputed());
    }

    #[test]
    fn resolved_clears_dispute() {
        let mut tx = Transaction::new(1, dec!(10), true);
        tx.resolved();
        assert!(!tx.disputed());
    }
}
