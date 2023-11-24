use camino::Utf8PathBuf;
use chrono::{DateTime, Utc};
use egui_extras::{Column, TableBuilder};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt, fs::{File, self}, io::Write, time::SystemTime, path::Path, error::Error};
use rfd::FileDialog;

use crate::{item::Item, order_state::OrderStatus};

#[derive(Default, Serialize, Deserialize, PartialEq)]
pub struct PurchaseOrder {
    pub po_number: u64,
    vendor: String,
    raised_by: String,
    date: Option<SystemTime>,
    department: String,
    items: Vec<Item>,
    pub current_status: OrderStatus,
    status_history: Vec<OrderStatus>,
    total_price: u64,
    invoice_price: u64,
    attached_files: HashMap<String, Vec<u8>>,
}
impl PurchaseOrder {
    pub fn new() -> Self {
        let date = Some(SystemTime::now());
        let raised_by = whoami::username();

        let po_number = Self::get_new_number().unwrap();

        Self {
            po_number,
            date,
            raised_by,
            ..Default::default()
        }
    }

    fn get_new_number() -> Result<u64, Box<dyn Error>> {
        //TODO; refactor this bad code to make it faster, stronger. better?
        let mut purchase_orders = Vec::default();

        for po_entry_dir in std::fs::read_dir("purchase_orders")? {
            let path = Utf8PathBuf::from_path_buf(po_entry_dir.unwrap().path()).unwrap();
            if path.is_dir() { 
                if let Some(number) = path.file_name() {
                    let dir = std::fs::read_to_string(format!("purchase_orders/{}/{}.json", number, number)).unwrap();
                    let po: PurchaseOrder = serde_json::from_str(&dir).unwrap();
                    purchase_orders.push(po)
                }
            }
        }

        let number = purchase_orders.iter().map(|x| x.po_number).collect::<Vec<u64>>().iter().max().cloned();

        if let Some(idx) = number { 
            Ok(idx + 1)
        } else { Ok(1) }
    }

    fn save(&mut self) {
        if !Path::new(&format!("purchase_orders/{}/", self.po_number)).is_dir() { 
            let mut folder = fs::create_dir(format!("purchase_orders/{}/", self.po_number)).unwrap();
        }
        let mut file = File::create(format!("purchase_orders/{}/{}.json", self.po_number, self.po_number)).unwrap();
        file.write_all(serde_json::to_string(self).unwrap().as_bytes());
    }
    pub fn order_status_ui(&mut self, ui: &mut egui::Ui) {
        ui.button("Revert Status");
    }

    pub fn file_list(&mut self, ui: &mut egui::Ui) {
        ui.heading("Uploaded files");
        egui::ScrollArea::vertical()
            .max_height(100.00)
            .show(ui, |ui| {
                let table = TableBuilder::new(ui)
                    .striped(true)
                    .resizable(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .columns(Column::auto(), 2);
                table
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                        });
                        header.col(|ui| {

                        });
                    }) 
                    .body(|mut body| {
                        &self.attached_files.keys().for_each(|file_name| {
                            body.row(20.0, |mut row| {
                                row.col(|ui| { ui.label(format!("{}", file_name )); });
                                row.col(|ui| { ui.button("Delete"); });
                            });
                        });
                    });
            });
    }

    pub fn file_uploader_ui(&mut self, ui: &mut egui::Ui) {



        if ui.button("Upload file").clicked() { 
            let file = FileDialog::new()
                .set_directory("/")
                .pick_file();

            if let Some(picked_file) = file { 
                let file_name = picked_file.file_name().unwrap().to_str().unwrap().to_string();
                let data = std::fs::read(&picked_file).unwrap();
                println!("{}", &file_name);
                self.attached_files.insert(file_name, data);
            }

        };
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Purchase order number");
            ui.monospace(format!("{}", self.po_number.clone()));
            ui.separator();
            ui.label(format!("Created on: {}", self.iso8601()));
            ui.separator();
            ui.label(format!("Created By: {}", self.raised_by));
        });
        ui.separator();

        //ui.set_enabled(self.status.current_status == StatusType::PendingApproval);
        ui.horizontal(|ui| {
            ui.vertical(|ui|{
                ui.label("Supplier:");
                ui.add(egui::TextEdit::multiline(&mut self.vendor).desired_width(50.0).hint_text("Input a supplier name, Please include address and contact details if possible.").desired_width(200.0));
            });
            ui.horizontal_top(|ui| {
                egui::ComboBox::from_label("Department")
                .selected_text(format!("{:?}", self.department))
                .show_ui(ui, |ui| {
                    for department in crate::DEPARTMENTS {
                        ui.selectable_value(
                            &mut self.department,
                            department.to_string(),
                            department.to_string(),
                        );
                    }
                });
            });
            });

        /*
        egui::ScrollArea::vertical()
            .max_height(100.00)
            .show(ui, |ui| {
                let table = TableBuilder::new(ui)
                    .striped(true)
                    .resizable(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .columns(Column::auto(), 5);
                table
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.strong("Index");
                        });
                        header.col(|ui| {
                            ui.strong("Qty");
                        });
                        header.col(|ui| {
                            ui.strong("Part Number");
                        });
                        header.col(|ui| {
                            ui.strong("Description");
                        });
                        header.col(|ui| {
                            ui.strong("Price per unit");
                        });
                    })
                    .body(|mut body| {
                        for (idx, item) in self.items.iter_mut().rev().enumerate() {
                            body.row(20.0, |mut row| {
                                row.col(|ui| {
                                    ui.label(format!("{idx}"));
                                });
                                row.col(|ui| {
                                    ui.add(egui::DragValue::new(&mut item.qty).speed(0.1));
                                });
                                row.col(|ui| {
                                    ui.text_edit_singleline(&mut item.part_number);
                                });
                                row.col(|ui| {
                                    ui.text_edit_singleline(&mut item.description);
                                });
                                row.col(|ui| {
                                    ui.add(egui::DragValue::new(&mut item.unit_price).speed(0.1));
                                });
                            });
                        }
                        body.row(10.0, |mut row| {
                            row.col(|ui| {
                                if ui.button("Add column").clicked() {
                                    self.items.push(Default::default());
                                }
                            });
                        });
                    });
            });
        */

        ui.separator();

        self.file_list(ui);
        self.file_uploader_ui(ui);

        //let total_price = self.items

        self.save();
    }

    fn iso8601(&self) -> String {
        if let Some(i) = self.date {
            let dt: DateTime<Utc> = i.clone().into();
            return format!("{}", dt.format("%+"));
        }
        "".to_string()
    }
}
