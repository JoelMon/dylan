use csv::StringRecord;
use serde::{Deserialize, Serialize};
use std::{fs::File, path::PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DErrors {
    #[error("File path has not been set")]
    EmptyPath,
}

// new time to represent a row
pub type Orders = Vec<Item>;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilePath(Option<PathBuf>);

impl FilePath {
    pub fn new(path: Option<PathBuf>) -> FilePath {
        FilePath(path)
    }

    pub fn get(self) -> Result<PathBuf, DErrors> {
        match self.0 {
            Some(p) => Ok(p),
            None => Err(DErrors::EmptyPath),
        }
    }
}

/// Item represents a single item, or line, within the ANS.csv file.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Item {
    pub ans: String,
    pub store: String,
    pub due_date: String,
    pub po: String,
    pub date_entered: String,
    pub fedex_tracking: String,
    pub upc: String,
    pub style: String,
    pub color: String,
    pub size: String,
    pub qty: String,
    pub completed_date: String,
    pub picker: String,
    pub oder_id: String,
}

/// The fields of an ANS.csv file.
pub enum Field {
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
    /// ```rust
    /// // The UPC field is located in the 6th index of the ANS.csv file.
    /// let f: usize = dylan::Field::UPC.index();
    ///
    /// assert_eq!(f, 6)
    /// ```
    pub fn index(&self) -> usize {
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

fn read_file(file_path: PathBuf) -> Result<Vec<StringRecord>, std::io::Error> {
    let file: File = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut records: Vec<StringRecord> = vec![];

    for result in rdr.records() {
        records.push(result?);
    }

    Ok(records)
}

/// Reads the csv file containing the information then passes it to
/// `clean()` to remove artifacts.
pub fn get_data(file_path: PathBuf) -> Result<Orders, std::io::Error> {
    let raw_data = read_file(file_path)?;
    println!("Still Reading from file");
    Ok(clean(raw_data).expect("Something went wrong while clean() ran")) // records after being cleaned and converted from StringRecord to an Item.
}

/// Cleans the data so that it can be processed.
///
/// Some of the fields within the ANS.csv file contain Excel artifacts.
/// The data within these fields are wrapped with `=""`.
/// This function removes these artifacts and leaves behind only the data.
pub fn clean(record: Vec<StringRecord>) -> Result<Vec<Item>, DErrors> {
    let item: Vec<Item> = record
        .iter()
        .map(|line| Item {
            ans: line
                .get(Field::Ans.index())
                .unwrap()
                .trim_start_matches("=\"")
                .trim_end_matches('\"')
                .to_string(),
            store: line
                .get(Field::DeliveryLocation.index())
                .unwrap()
                .trim_start_matches("=\"")
                .trim_end_matches('\"')
                .to_string(),
            due_date: line.get(Field::DateDue.index()).unwrap().to_string(),
            po: line
                .get(Field::PO.index())
                .unwrap()
                .trim_start_matches("=\"")
                .trim_end_matches('\"')
                .to_string(),
            date_entered: line
                .get(Field::DateEntered.index())
                .unwrap()
                .trim_start_matches("=\"")
                .trim_end_matches('\"')
                .to_string(),
            fedex_tracking: line
                .get(Field::FedExTracking.index())
                .unwrap()
                .trim_start_matches("=\"")
                .trim_end_matches('\"')
                .to_string(),
            upc: line
                .get(Field::UPC.index())
                .unwrap()
                .trim_start_matches("=\"")
                .trim_end_matches('\"')
                .to_string(),
            style: line
                .get(Field::Style.index())
                .unwrap()
                .trim_start_matches("=\"")
                .trim_end_matches('\"')
                .to_string(),
            color: line
                .get(Field::Color.index())
                .unwrap()
                .trim_start_matches("=\"")
                .trim_end_matches('\"')
                .to_string(),
            size: line
                .get(Field::Size.index())
                .unwrap()
                .trim_start_matches("=\"")
                .trim_end_matches('\"')
                .to_string(),
            qty: line.get(Field::Qty.index()).unwrap().to_string(),
            completed_date: line
                .get(Field::DateCompleted.index())
                .unwrap()
                .trim_start_matches("=\"")
                .trim_end_matches('\"')
                .to_string(),
            picker: line.get(Field::Picker.index()).unwrap().to_string(),
            oder_id: line.get(Field::OrderID.index()).unwrap().to_string(),
        })
        .collect();

    Ok(item)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn _t_field_index() {
        let po: usize = 3;
        let upc: usize = 6;

        assert_eq!(Field::PO.index(), po);
        assert_eq!(Field::UPC.index(), upc);
    }

    #[test]
    fn t_clean() {
        let dirty_item = vec![
            "=\"123\"",
            "=\"010\"",
            "6/8/2022 12:00:55 AM",
            "O0435NGTEE-010\"",
            "05/31/2022\"",
            "=\"580777777777\"",
            "=\"195333333333\"",
            "=\"67222222\"",
            "=\"Black\"",
            "=\"2XL\"",
            "5",
            "=\"06/08/2022\"",
            "240",
            "46984",
        ];

        let clean_item = Item {
            ans: String::from("123"),
            store: String::from("010"),
            due_date: String::from("6/8/2022 12:00:55 AM"),
            po: String::from("O0435NGTEE-010"),
            date_entered: String::from("05/31/2022"),
            fedex_tracking: String::from("580777777777"),
            upc: String::from("195333333333"),
            style: String::from("67222222"),
            color: String::from("Black"),
            size: String::from("2XL"),
            qty: String::from("5"),
            completed_date: String::from("06/08/2022"),
            picker: String::from("240"),
            oder_id: String::from("46984"),
        };

        let ser = vec![StringRecord::from(dirty_item)];

        assert_eq!(clean_item, clean(ser).unwrap()[0]);
    }
}
