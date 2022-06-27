use dylan::Item;
use eframe::egui;
use egui::Ui;
use egui_extras::{Size, TableBody};

pub fn table_ui(ctx: &egui::Context, _frame: &mut eframe::Frame, ui: &mut Ui, data: Vec<Item>) {
    egui_extras::TableBuilder::new(ui)
        .resizable(true)
        .column(Size::Absolute {
            initial: (40.00),
            range: (45.0, 100.0),
        })
        .column(Size::remainder().at_least(100.0))
        .column(Size::remainder().at_least(100.0))
        .column(Size::remainder().at_least(100.0))
        .column(Size::remainder().at_least(100.0))
        .column(Size::remainder().at_least(100.0))
        .column(Size::remainder().at_least(100.0))
        .column(Size::remainder().at_least(100.0))
        .column(Size::remainder().at_least(100.0))
        .column(Size::remainder().at_least(100.0))
        .column(Size::remainder().at_least(100.0))
        .column(Size::remainder().at_least(100.0))
        .column(Size::remainder().at_least(100.0))
        .column(Size::remainder().at_least(100.0))
        .column(Size::remainder())
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.heading("Row");
            });
            header.col(|ui| {
                ui.heading("ANS");
            });
            header.col(|ui| {
                ui.heading("Delivery Location");
            });
            header.col(|ui| {
                ui.heading("Due Date");
            });
            header.col(|ui| {
                ui.heading("PO");
            });
            header.col(|ui| {
                ui.heading("Date Entered");
            });
            header.col(|ui| {
                ui.heading("FedEx Number");
            });
            header.col(|ui| {
                ui.heading("UPC");
            });
            header.col(|ui| {
                ui.heading("Style");
            });
            header.col(|ui| {
                ui.heading("Color");
            });
            header.col(|ui| {
                ui.heading("Size");
            });
            header.col(|ui| {
                ui.heading("QTY");
            });
            header.col(|ui| {
                ui.heading("Completed");
            });
            header.col(|ui| {
                ui.heading("Picker");
            });
            header.col(|ui| {
                ui.heading("Order ID");
            });
        })
        .body(|mut body| {
            // Loads each row from `data`
            for item in data.iter().enumerate() {
                body.row(30.0, |mut row| {
                    row.col(|ui| {
                        ui.label(item.0.to_string());
                    });
                    row.col(|ui| {
                        ui.label(item.1.ans.to_string());
                    });
                    row.col(|ui| {
                        ui.label(item.1.store.to_string());
                    });
                    row.col(|ui| {
                        ui.label(item.1.due_date.to_string());
                    });
                    row.col(|ui| {
                        ui.label(item.1.po.to_string());
                    });
                    row.col(|ui| {
                        ui.label(item.1.date_entered.to_string());
                    });
                    row.col(|ui| {
                        ui.label(item.1.fedex_tracking.to_string());
                    });
                    row.col(|ui| {
                        ui.label(item.1.upc.to_string());
                    });
                    row.col(|ui| {
                        ui.label(item.1.style.to_string());
                    });
                    row.col(|ui| {
                        ui.label(item.1.color.to_string());
                    });
                    row.col(|ui| {
                        ui.label(item.1.size.to_string());
                    });
                    row.col(|ui| {
                        ui.label(item.1.qty.to_string());
                    });
                    row.col(|ui| {
                        ui.label(item.1.completed_date.to_string());
                    });
                    row.col(|ui| {
                        ui.label(item.1.picker.to_string());
                    });
                    row.col(|ui| {
                        ui.label(item.1.oder_id.to_string());
                    });
                })
            }
        });
}
