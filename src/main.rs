use std::collections::HashSet;
use std::{fs::File, path::PathBuf};

use anyhow::{Context, Result};
use csv::{self, StringRecord};
use serde::{Deserialize, Serialize};

use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FedEx {
    #[serde(rename = "tracking_number")]
    pub tracking_number: String,
    #[serde(rename = "tracking_url")]
    pub tracking_url: String,
    #[serde(rename = "status_code")]
    pub status_code: String,
    #[serde(rename = "carrier_code")]
    pub carrier_code: String,
    #[serde(rename = "carrier_id")]
    pub carrier_id: i64,
    #[serde(rename = "carrier_detail_code")]
    pub carrier_detail_code: Value,
    #[serde(rename = "status_description")]
    pub status_description: String,
    #[serde(rename = "carrier_status_code")]
    pub carrier_status_code: String,
    #[serde(rename = "carrier_status_description")]
    pub carrier_status_description: String,
    #[serde(rename = "ship_date")]
    pub ship_date: Value,
    #[serde(rename = "estimated_delivery_date")]
    pub estimated_delivery_date: Value,
    #[serde(rename = "actual_delivery_date")]
    pub actual_delivery_date: Value,
    #[serde(rename = "exception_description")]
    pub exception_description: Value,
    pub events: Vec<Event>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    #[serde(rename = "occurred_at")]
    pub occurred_at: String,
    #[serde(rename = "carrier_occurred_at")]
    pub carrier_occurred_at: String,
    pub description: String,
    #[serde(rename = "city_locality")]
    pub city_locality: String,
    #[serde(rename = "state_province")]
    pub state_province: String,
    #[serde(rename = "postal_code")]
    pub postal_code: String,
    #[serde(rename = "country_code")]
    pub country_code: String,
    #[serde(rename = "company_name")]
    pub company_name: String,
    pub signer: String,
    #[serde(rename = "event_code")]
    pub event_code: String,
    #[serde(rename = "carrier_detail_code")]
    pub carrier_detail_code: Value,
    #[serde(rename = "status_code")]
    pub status_code: Value,
    #[serde(rename = "status_description")]
    pub status_description: Value,
    #[serde(rename = "carrier_status_code")]
    pub carrier_status_code: String,
    #[serde(rename = "carrier_status_description")]
    pub carrier_status_description: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

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
