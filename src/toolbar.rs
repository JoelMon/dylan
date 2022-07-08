use dylan::Item;
use eframe::egui;
use egui::{global_dark_light_mode_switch, Ui};
use rfd;

use crate::gui::Gui;

pub fn toolbar(
    items: &mut Vec<Item>,
    ctx: &egui::Context,
    _frame: &mut eframe::Frame,
    _ui: &mut Ui,
) {
    egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
        egui::Frame::none().show(ui, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::menu::menu_button(ui, "File", |ui| {
                    // File Open
                    // TODO: Prevent crash when user cancels file dialog box
                    if ui.button("Open").clicked() {
                        let path = rfd::FileDialog::new()
                            .add_filter("csv", &["csv"])
                            .add_filter("CSV", &["CSV"])
                            .pick_file();
                        Gui::load_file(items, path.unwrap());
                    }
                    if ui.button("Exit").clicked() {
                        panic!("Exit button clicked: Replace with proper exit code");
                    }
                });

                egui::menu::menu_button(ui, "Filters", |ui| {
                    if ui.button("Picker 128").clicked() {
                        println!("Picker 128 was clicked.");

                        todo!();
                    }
                });
                ui.separator();
                global_dark_light_mode_switch(ui);
            });
        });
    });
}
