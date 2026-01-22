use rust_decimal_macros::dec;
use transaction_processing_engine::{common::InputData, engine::run_engine};

#[tokio::test]
async fn single_client_dispute_flow() {
    let (tx, rx) = tokio::sync::mpsc::channel(10);

    tx.send(InputData::deposit(1, 1, dec!(10))).await.unwrap();
    tx.send(InputData::withdrawal(1, 2, dec!(5))).await.unwrap();
    tx.send(InputData::dispute(1, 2)).await.unwrap();
    tx.send(InputData::resolve(1, 2)).await.unwrap();
    drop(tx); // close channel

    let accounts = run_engine(rx).await;
    let acc = &accounts[&1];

    assert_eq!(acc.available, dec!(5));
    assert_eq!(acc.held, dec!(0));
    assert_eq!(acc.total, dec!(5));
    assert!(!acc.locked);
}

#[tokio::test]
async fn run_engine_end_to_end() {
    let (tx, rx) = tokio::sync::mpsc::channel(10);

    tx.send(InputData::deposit(1, 1, dec!(10))).await.unwrap();
    tx.send(InputData::withdrawal(1, 2, dec!(5))).await.unwrap();
    tx.send(InputData::dispute(1, 2)).await.unwrap();
    tx.send(InputData::resolve(1, 2)).await.unwrap();
    drop(tx);

    let accounts = run_engine(rx).await;
    let acc = &accounts[&1];

    assert_eq!(acc.available, dec!(5));
    assert_eq!(acc.total, dec!(5));
    assert!(!acc.locked);
}
