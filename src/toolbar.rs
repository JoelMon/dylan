use crate::gui::{Gui, Push};
use dylan::{FilePath, Item};
use eframe::egui;
use egui::{global_dark_light_mode_switch, Ui};
use rfd;

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
                        println!("Picker 128 was clicked.");

                        let new_item = Item {
                            ans: String::from("1244545454543"),
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

                        items.push(new_item);
                        println!("Pushed");
                    }
                });
                ui.separator();
                global_dark_light_mode_switch(ui);
            });
        });
    });
}
