use anyhow::Ok;
use dylan::FilePath;
use eframe::egui;
use egui::{global_dark_light_mode_switch, Ui};
use rfd;
use std::path::PathBuf;

pub fn toolbar(ctx: &egui::Context, _frame: &mut eframe::Frame, _ui: &mut Ui) {
    egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
        egui::Frame::none().show(ui, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::menu::menu_button(ui, "File", |ui| {
                    if ui.button("Open").clicked() {
                        let path = FilePath::new(
                            rfd::FileDialog::new()
                                .add_filter("CSV", &["csv"])
                                .pick_file(),
                        );
                        dbg!(path);
                    }
                    if ui.button("Exit").clicked() {
                        panic!("Exit button clicked: Replace with proper exit code");
                    }
                });
                egui::menu::menu_button(ui, "Filters", |ui| {
                    if ui.button("Picker 128").clicked() {
                        // filter_chooser(ctx, _frame, ui);
                    };
                });
                ui.separator();
                global_dark_light_mode_switch(ui);
            });
        });
    });
}
