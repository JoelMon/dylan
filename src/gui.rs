use crate::toolbar;
use dylan::{get_data, Item, Orders};
use eframe::egui;
use egui_extras::Size;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Gui {
    /// Items loaded from csv file
    pub(crate) items: Orders,
}

impl Gui {
    pub fn load_file(items: &mut Orders, path: PathBuf) {
        items.clear();

        for item in get_data(path).unwrap() {
            items.push(item);
        }
    }
    pub fn get(&self) -> Orders {
        self.items.clone()
    }
}

impl Default for Gui {
    fn default() -> Self {
        Self {
            items: vec![Item {
                ans: String::from(""),
                store: String::from(""),
                due_date: String::from(""),
                po: String::from(""),
                date_entered: String::from(""),
                fedex_tracking: String::from(""),
                upc: String::from(""),
                style: String::from(""),
                color: String::from(""),
                size: String::from(""),
                qty: String::from(""),
                completed_date: String::from(""),
                picker: String::from(""),
                oder_id: String::from(""),
            }],
        }
    }
}

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
            let win = Filters { open: false };
            if win.get() {
                egui::Window::new("My Window").show(ctx, |ui| {
                    ui.label("Hello World!");
                });
            }
            toolbar::toolbar(&mut self.items, ctx, _frame, ui);
            ui.add_space(30.5);

            // ##################TABLE##############################//
            let data = self;
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
                    for row_index in data.get().into_iter().enumerate() {
                        body.row(30.0, |mut row| {
                            row.col(|ui| {
                                ui.label(row_index.0.to_string());
                            });
                            row.col(|ui| {
                                ui.label(row_index.1.ans);
                            });
                            row.col(|ui| {
                                ui.label(row_index.1.store);
                            });
                            row.col(|ui| {
                                ui.label(row_index.1.due_date);
                            });
                            row.col(|ui| {
                                ui.label(row_index.1.po);
                            });
                            row.col(|ui| {
                                ui.label(row_index.1.date_entered);
                            });
                            row.col(|ui| {
                                ui.label(row_index.1.fedex_tracking);
                            });
                            row.col(|ui| {
                                ui.label(row_index.1.upc);
                            });
                            row.col(|ui| {
                                ui.label(row_index.1.style);
                            });
                            row.col(|ui| {
                                ui.label(row_index.1.color);
                            });
                            row.col(|ui| {
                                ui.label(row_index.1.size);
                            });
                            row.col(|ui| {
                                ui.label(row_index.1.qty);
                            });
                            row.col(|ui| {
                                ui.label(row_index.1.completed_date);
                            });
                            row.col(|ui| {
                                ui.label(row_index.1.picker);
                            });
                            row.col(|ui| {
                                ui.label(row_index.1.oder_id);
                            });
                        })
                    }
                })
        });
    }
}
