use crate::{common::InputData, errors::ParseError};
use csv::ReaderBuilder;
use std::{env, fs::File};

pub async fn process_file_data(tx: tokio::sync::mpsc::Sender<InputData>) -> Result<(), ParseError> {
    // read data from csv file and send to the csvdata channel, source can be changed to be something other than a file

    let filename = env::args()
        .skip(1) // skip the program name
        .next()
        .unwrap_or(String::from("transactions.csv"));

    let file = File::open(&filename)?;

    // Build the CSV reader and iterate over each record.
    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .trim(csv::Trim::All)
        .from_reader(file);

    for result in reader.deserialize::<InputData>() {
        let record = result?;

        tx.send(record)
            .await
            .map_err(|err| ParseError::Channel(err))?;
    }
    Ok(())
}
