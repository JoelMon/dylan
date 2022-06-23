use std::{fs::File, path::PathBuf};

mod fedex;
use anyhow::{Context, Result};
use csv::{self, StringRecord};

fn read_file(file_path: PathBuf) -> Result<Vec<StringRecord>> {
    let file = File::open(file_path).context("Failed to open file")?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut records: Vec<StringRecord> = vec![];

    for result in rdr.records() {
        records.push(result?);
    }

    Ok(records)
}

fn run() -> anyhow::Result<()> {
    let file_path = PathBuf::from(r"C:\Users\RFID\Desktop\ANS.CSV");
    let record = read_file(file_path);
    let _record = dylan::clean(record?)?; // records after being cleaned

    let number: usize = 5555555555;

    let fx = fedex::Tracking::add(number)
        .context("Something went wrong while entering the FedEx tracking number.");

    print!("{:?}", fx);

    Ok(())
}
fn main() {
    let _ = run();
}
