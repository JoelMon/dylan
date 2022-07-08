use eframe::egui;
use egui::Ui;

/// a demo of how to separate windown into different mods
pub fn win2(ctx: &egui::Context, _frame: &mut eframe::Frame, _ui: &mut Ui) {
    egui::Window::new("My Window2").show(ctx, |ui| {
        ui.label("Finally!");
    });
    // egui::Frame::none().fill(egui::Color32::RED).show(ui, |ui| {
    //     ui.label("Label with red background");
    // });
}
