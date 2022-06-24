use crate::table;
use crate::toolbar;
use crate::win;
use eframe::egui;

#[derive(Debug, Default)]
pub struct Gui;

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // egui::Window::new("My Window").show(ctx, |ui| {
            //     ui.label("Hello World!");
            // });
            // win::win2(ctx, _frame, ui);
            toolbar::toolbar(ctx, _frame, ui);
        });
    }
}
