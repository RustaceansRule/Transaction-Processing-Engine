use transaction_processing_engine::{common::InputData, engine, source};

#[tokio::main]
async fn main() {
    let (tx, rx) = tokio::sync::mpsc::channel::<InputData>(10_000);

    let handle = tokio::spawn(async move { source::process_file_data(tx).await });
    let accounts = engine::run_engine(rx).await;
    let _ = handle.await;

    transaction_processing_engine::output_data(&accounts);
}
