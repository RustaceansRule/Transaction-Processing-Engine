use crate::errors::AccountingError;
use rust_decimal::Decimal;

#[derive(Debug, Default)]
pub struct Account {
    pub available: Decimal,
    pub held: Decimal,
    pub total: Decimal,
    pub locked: bool,
}

impl Account {
    pub fn deposit(&mut self, amount: Decimal) {
        self.available += amount;
        self.total += amount
    }

    pub fn dispute(&mut self, amount: Decimal) {
        self.available -= amount;
        self.held += amount;
    }

    pub fn resolve(&mut self, amount: Decimal) {
        self.held -= amount;
        self.available += amount;
    }

    pub fn withdraw(&mut self, amount: Decimal) -> Result<(), AccountingError> {
        match self.available - amount >= Decimal::ZERO {
            true => {
                self.available -= amount;
                self.total -= amount;
                Ok(())
            }
            false => Err(AccountingError::InsufficientFunds),
        }
    }

    pub fn chargeback(&mut self, amount: Decimal) {
        self.held -= amount;
        self.total -= amount;
        self.locked = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn deposit_increases_available_and_total() {
        let mut acc = Account::default();
        acc.deposit(dec!(10));

        assert_eq!(acc.available, dec!(10));
        assert_eq!(acc.total, dec!(10));
        assert_eq!(acc.held, dec!(0));
        assert!(!acc.locked);
    }

    #[test]
    fn withdraw_success() {
        let mut acc = Account::default();
        acc.deposit(dec!(10));

        acc.withdraw(dec!(4)).unwrap();

        assert_eq!(acc.available, dec!(6));
        assert_eq!(acc.total, dec!(6));
    }

    #[test]
    fn withdraw_insufficient_funds_fails() {
        let mut acc = Account::default();
        acc.deposit(dec!(5));

        let res = acc.withdraw(dec!(10));
        assert_eq!(res, Err(AccountingError::InsufficientFunds));

        assert_eq!(acc.available, dec!(5));
        assert_eq!(acc.total, dec!(5));
    }

    #[test]
    fn total_equals_available_plus_held() {
        let mut acc = Account::default();
        acc.deposit(dec!(20));
        acc.withdraw(dec!(7)).unwrap();

        assert_eq!(acc.total, acc.available + acc.held);
    }
}
