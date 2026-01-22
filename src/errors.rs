use thiserror::Error;
use tokio::sync::mpsc::error::SendError;

use crate::common::InputData;

#[derive(Error, Debug, PartialEq)]
pub enum AccountingError {
    #[error("Not enough funds available")]
    InsufficientFunds,
    #[error("Account locked")]
    AccountLocked,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("IO error")]
    IO(#[from] std::io::Error),

    #[error("CSV File error")]
    Csv(#[from] csv::Error),

    #[error("Channel send error")]
    Channel(#[from] SendError<InputData>),
}
