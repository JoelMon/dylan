use std::{fs::File, path::PathBuf};
mod fedex;
mod gui;
mod table;
mod toolbar;
mod win;
use anyhow::{Context, Result};
use csv::{self, StringRecord};
use eframe::egui;

fn read_file(file_path: PathBuf) -> Result<Vec<StringRecord>> {
    let file = File::open(file_path).context("Failed to open file");
    let mut rdr = csv::Reader::from_reader(file?);
    let mut records: Vec<StringRecord> = vec![];

    for result in rdr.records() {
        records.push(result?);
    }

    Ok(records)
}
fn run_gui() {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        ..Default::default()
    };
    eframe::run_native(
        "LISA",
        options,
        Box::new(|_cc| Box::new(gui::Gui::default())),
    );
}

fn run() -> Result<()> {
    let file_path = PathBuf::from(r"C:\Users\RFID\Desktop\ANS.CSV");
    // let file_path = PathBuf::from(r"/home/joel/Downloads/ANS.CSV");
    let raw_data = read_file(file_path).context("Error opening the file")?;
    let cleaned_data = dylan::clean(raw_data)?; // records after being cleaned and converted from StringRecord to an Item.

    run_gui();
    Ok(())
}
fn main() {
    run().expect("An error has been triggered");
}
