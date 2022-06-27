use crate::get_data;
use crate::table;
use crate::toolbar;
use eframe::egui;

#[derive(Debug, Default)]
pub struct Gui;

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // egui::Window::new("My Window").show(ctx, |ui| {
            //     ui.label("Hello World!");
            // });
            toolbar::toolbar(ctx, _frame, ui);
            ui.add_space(30.5);
            table::table_ui(ctx, _frame, ui, get_data().unwrap());
        });
    }
}
