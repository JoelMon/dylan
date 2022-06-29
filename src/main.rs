use std::{fs::File, path::PathBuf};
mod fedex;
mod filters;
mod gui;
mod toolbar;
mod win;
use anyhow::{Context, Result};

fn run_gui() {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        ..Default::default()
    };
    eframe::run_native(
        "Dylan",
        options,
        Box::new(|_cc| Box::new(gui::Gui::default())),
    );
}

fn run() -> Result<()> {
    run_gui();
    Ok(())
}
fn main() {
    run().expect("An error has been triggered");
}
