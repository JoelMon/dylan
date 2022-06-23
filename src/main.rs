use std::collections::HashSet;
use std::{fs::File, path::PathBuf};

use anyhow::{Context, Result};
use csv::{self, StringRecord};
use serde::{Deserialize, Serialize};

/// The fields of an ANS.csv file.
enum Field {
    Ans,
    DeliveryLocation,
    DateDue,
    PO,
    DateEntered,
    FedExTracking,
    UPC,
    Style,
    Color,
    Size,
    Qty,
    DateCompleted,
    Picker,
    OrderID,
}

impl Field {
    /// Returns the CSV index for the corresponding field being passed in.
    ///
    /// # Example
    /// ```rust, ignore
    /// let f: Field = Field::UPC.index();
    ///
    /// assert_eq!(f, 6)
    /// ```
    fn index(&self) -> usize {
        match &self {
            Field::Ans => 0,
            Field::DeliveryLocation => 1,
            Field::DateDue => 2,
            Field::PO => 3,
            Field::DateEntered => 4,
            Field::FedExTracking => 5,
            Field::UPC => 6,
            Field::Style => 7,
            Field::Color => 8,
            Field::Size => 9,
            Field::Qty => 10,
            Field::DateCompleted => 11,
            Field::Picker => 12,
            Field::OrderID => 13,
        }
    }
}

fn clean(record: Vec<StringRecord>) -> Result<Vec<Report>> {
    let mut item: Vec<Report> = record
        .iter()
        .map(|line| Report {
            ans: line
                .get(Field::Ans.index())
                .unwrap()
                .trim_start_matches("=\"")
                .trim_end_matches("\"")
                .to_string(),
            store: line
                .get(Field::DeliveryLocation.index())
                .unwrap()
                .trim_start_matches("=\"")
                .trim_end_matches("\"")
                .to_string(),
            due_date: line.get(Field::DateDue.index()).unwrap().to_string(),
            po: line
                .get(Field::PO.index())
                .unwrap()
                .trim_start_matches("=\"")
                .trim_end_matches("\"")
                .to_string(),
            date_entered: line
                .get(Field::DateEntered.index())
                .unwrap()
                .trim_start_matches("=\"")
                .trim_end_matches("\"")
                .to_string(),
            fedex_tracking: line
                .get(Field::FedExTracking.index())
                .unwrap()
                .trim_start_matches("=\"")
                .trim_end_matches("\"")
                .to_string(),
            upc: line
                .get(Field::UPC.index())
                .unwrap()
                .trim_start_matches("=\"")
                .trim_end_matches("\"")
                .to_string(),
            style: line
                .get(Field::Style.index())
                .unwrap()
                .trim_start_matches("=\"")
                .trim_end_matches("\"")
                .to_string(),
            color: line
                .get(Field::Color.index())
                .unwrap()
                .trim_start_matches("=\"")
                .trim_end_matches("\"")
                .to_string(),
            size: line
                .get(Field::Size.index())
                .unwrap()
                .trim_start_matches("=\"")
                .trim_end_matches("\"")
                .to_string(),
            qty: line.get(Field::Qty.index()).unwrap().to_string(),
            completed_date: line
                .get(Field::DateCompleted.index())
                .unwrap()
                .trim_start_matches("=\"")
                .trim_end_matches("\"")
                .to_string(),
            picker: line.get(Field::Picker.index()).unwrap().to_string(),
            oder_id: line.get(Field::OrderID.index()).unwrap().to_string(),
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
