use std::collections::HashMap;

pub mod common;
pub mod engine;
pub mod errors;
pub mod models;
pub mod source;

pub fn output_data(accounts: &HashMap<u16, models::account::Account>) {
    println!("client, available, held, total, locked");
    accounts.iter().for_each(|account| {
        println!(
            "{},{},{},{},{}",
            account.0, account.1.available, account.1.held, account.1.total, account.1.locked
        )
    });
}
