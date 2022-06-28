use crate::table;
use crate::toolbar;
use dylan::{get_data, Item};
use eframe::egui;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct Gui;

#[derive(Debug, Default)]
pub struct Filters {
    pub open: bool,
}
impl Filters {
    pub fn get(&self) -> bool {
        self.open
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let win = Filters { open: true };
            if win.get() {
                egui::Window::new("My Window").show(ctx, |ui| {
                    ui.label("Hello World!");
                });
            }
            toolbar::toolbar(ctx, _frame, ui);
            ui.add_space(30.5);
            table::table_ui(ctx, _frame, ui); // TODO: Slowness is due the processing of data with each frame refresh
        });
    }
}
