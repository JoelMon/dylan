use anyhow::{Context, Result};
use csv::StringRecord;
use serde::{Deserialize, Serialize};

/// Represents a single item or line within the ANS.csv file.
#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
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

pub fn clean(record: Vec<StringRecord>) -> Result<Vec<Item>> {
    let mut item: Vec<Item> = record
        .iter()
        .map(|line| Item {
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
