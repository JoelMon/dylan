use eframe::egui;
use egui::Ui;
use egui_extras::{Size, TableBody};

pub fn table_ui(ctx: &egui::Context, _frame: &mut eframe::Frame, ui: &mut Ui) {
    egui_extras::TableBuilder::new(ui)
        .resizable(true)
        .column(Size::remainder().at_least(100.0))
        .column(Size::remainder().at_least(100.0))
        .column(Size::exact(60.0))
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.heading("Growing");
            });
            header.col(|ui| {
                ui.heading("Second Header");
            });
            header.col(|ui| {
                ui.heading("Fixed");
            });
        })
        .body(|mut body| {
            body.row(30.0, |mut row| {
                row.col(|ui| {
                    ui.label("first row growing cell");
                });
                row.col(|ui| {
                    ui.label("Second row growing cell");
                });
                row.col(|ui| {
                    ui.button("action");
                });
            });
        });
}
