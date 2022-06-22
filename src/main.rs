use std::collections::HashSet;
use std::{fs::File, path::PathBuf};

use anyhow::{Context, Result};
use csv::{self, StringRecord};
use serde::{Deserialize, Serialize};

fn clean(record: Vec<StringRecord>) -> Result<Vec<Report>> {
    let mut item: Vec<Report> = record
        .iter()
        .map(|line| Report {
            ans: line
                .get(0)
                .unwrap()
                .trim_start_matches("=\"")
                .trim_end_matches("\"")
                .to_string(),
            store: line
                .get(1)
                .unwrap()
                .trim_start_matches("=\"")
                .trim_end_matches("\"")
                .to_string(),
            due_date: line.get(2).unwrap().to_string(),
            po: line
                .get(3)
                .unwrap()
                .trim_start_matches("=\"")
                .trim_end_matches("\"")
                .to_string(),
            date_entered: line
                .get(4)
                .unwrap()
                .trim_start_matches("=\"")
                .trim_end_matches("\"")
                .to_string(),
            fedex_tracking: line
                .get(5)
                .unwrap()
                .trim_start_matches("=\"")
                .trim_end_matches("\"")
                .to_string(),
            upc: line
                .get(6)
                .unwrap()
                .trim_start_matches("=\"")
                .trim_end_matches("\"")
                .to_string(),
            style: line
                .get(7)
                .unwrap()
                .trim_start_matches("=\"")
                .trim_end_matches("\"")
                .to_string(),
            color: line
                .get(8)
                .unwrap()
                .trim_start_matches("=\"")
                .trim_end_matches("\"")
                .to_string(),
            size: line
                .get(9)
                .unwrap()
                .trim_start_matches("=\"")
                .trim_end_matches("\"")
                .to_string(),
            qty: line.get(10).unwrap().to_string(),
            completed_date: line
                .get(11)
                .unwrap()
                .trim_start_matches("=\"")
                .trim_end_matches("\"")
                .to_string(),
            picker: line.get(12).unwrap().to_string(),
            oder_id: line.get(13).unwrap().to_string(),
        })
        .collect();

    Ok(item)
}

#[derive(Debug, Deserialize, Serialize)]
struct Report {
    ans: String,
    store: String,
    due_date: String,
    po: String,
    date_entered: String,
    fedex_tracking: String,
    upc: String,
    style: String,
    color: String,
    size: String,
    qty: String,
    completed_date: String,
    picker: String,
    oder_id: String,
}

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
    let record = clean(record?)?; // records after being cleaned

    // Collect all FedEx tracking information and store unique.
    let mut fedex: HashSet<String> = record.iter().map(|x| x.fedex_tracking.to_owned()).collect();

    dbg!(fedex);

    Ok(())
}
fn main() {
    let _ = run();
}
