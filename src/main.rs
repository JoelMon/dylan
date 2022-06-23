use std::{fs::File, path::PathBuf};

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

fn run() -> Result<()> {
    let file_path = PathBuf::from(r"C:\Users\RFID\Desktop\ANS.CSV");
    let record = read_file(file_path);
    let record = dylan::clean(record?)?; // records after being cleaned

    // Collect all FedEx tracking information and store unique.
    // let mut fedex: HashSet<String> = record.iter().map(|x| x.fedex_tracking.to_owned()).collect();

    dbg!(record);

    // dbg!(fedex);

    Ok(())
}
fn main() {
    let _ = run();
}
