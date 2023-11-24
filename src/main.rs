use camino::Utf8PathBuf;
use egui::Visuals;
use order_state::{OrderStatus, Signature};
use purchase_order::PurchaseOrder;

pub mod item;
mod order_state;
mod purchase_order;

const DEPARTMENTS: &[&str] = &[
    "Specials",
    "Production Enginering",
    "Grinding",
    "Quality",
    "Factory",
];

#[derive(PartialEq)]
struct PoApp {
    purchase_orders: Vec<PurchaseOrder>,
    po_index_focus: Option<usize>,
    status_filter: Option<OrderStatus>,
}

impl PoApp {
    fn default() -> Self {
        let purchase_orders = Self::load_purchase_orders();
        Self {
            purchase_orders,
            po_index_focus: None,
            status_filter: None,
        }
    }

    fn load_purchase_orders() -> Vec<PurchaseOrder> {
        //todo; refactor this code to make it nicer because right now its not very nice
        let mut purchase_orders = Vec::default();

        for po_entry_dir in std::fs::read_dir("purchase_orders").unwrap() {
            let path = Utf8PathBuf::from_path_buf(po_entry_dir.unwrap().path()).unwrap();
            if path.is_dir() { 
                if let Some(number) = path.file_name() {
                    let dir = std::fs::read_to_string(format!("purchase_orders/{}/{}.json", number, number)).unwrap();
                    let po: PurchaseOrder = serde_json::from_str(&dir).unwrap();
                    purchase_orders.push(po)
                }
            }
        }

        /*for dir_entry in purchase_files {
            let path = dir_entry.unwrap().path();
            let folder = std::fs::read_dir(path).unwrap();
            let json_entry = std::fs::read_to_string(folder).unwrap();
            let po: PurchaseOrder = serde_json::from_str(&json_entry).unwrap();
            purchase_orders.push(po);
        }*/
        purchase_orders
    }
}

impl eframe::App for PoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(Visuals::dark());
        ctx.set_pixels_per_point(1.2);
        self.purchase_orders.sort_by_key(|k| k.po_number);

        egui::SidePanel::left("purchase_order_list")
            .resizable(false)
            .exact_width(169.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Create Purchase Order 💸").clicked() {
                        //create new purchase order object
                        self.purchase_orders.push(PurchaseOrder::new());
                        //set focus to the created purchase order
                        self.po_index_focus = Some(self.purchase_orders.len() - 1);
                    }
                    if ui.button("🔎").clicked() {
                        if self.status_filter != None {
                            self.status_filter = None
                        } else {
                            self.status_filter = Some(OrderStatus::Unapproved)
                        }
                    }
                });

                if self.status_filter != None {
                    egui::ComboBox::from_label("Status filter")
                        .selected_text(format!("{}", self.status_filter.as_ref().unwrap()))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.status_filter,
                                Some(OrderStatus::Unapproved),
                                "Unapproved",
                            );
                            ui.selectable_value(
                                &mut self.status_filter,
                                Some(OrderStatus::Approved(Signature::none())),
                                "Approved",
                            );
                            ui.selectable_value(
                                &mut self.status_filter,
                                Some(OrderStatus::Ordered(Signature::none())),
                                "Ordered",
                            );
                            ui.selectable_value(
                                &mut self.status_filter,
                                Some(OrderStatus::Recieved(Signature::none())),
                                "Recieved",
                            );
                            ui.selectable_value(
                                &mut self.status_filter,
                                Some(OrderStatus::Paid(Signature::none())),
                                "Paid",
                            );
                            ui.selectable_value(
                                &mut self.status_filter,
                                Some(OrderStatus::OrderComplete),
                                "Completed",
                            );
                        });
                }

                ui.separator();

                egui::ScrollArea::vertical()
                    .max_width(f32::INFINITY)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for (idx, purchase_order) in self.purchase_orders.iter().rev().enumerate() {
                            //TODO: refactor this block of code (it is very messy)
                            if self.status_filter != None {
                                if Some(&purchase_order.current_status)
                                    == self.status_filter.as_ref()
                                {
                                    if ui
                                        .button(format!(
                                            "{} - {}",
                                            purchase_order.po_number, purchase_order.current_status
                                        ))
                                        .clicked()
                                    {
                                        self.po_index_focus =
                                            Some(self.purchase_orders.len() - idx - 1);
                                    };
                                }
                            } else {
                                if ui
                                    .button(format!(
                                        "{} - {}",
                                        purchase_order.po_number, purchase_order.current_status
                                    ))
                                    .clicked()
                                {
                                    self.po_index_focus =
                                        Some(self.purchase_orders.len() - idx - 1);
                                };
                            }
                        }
                    });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(idx) = self.po_index_focus {
                let po = &mut self.purchase_orders[idx].ui(ui);
            };
        });
        egui::SidePanel::right("status_manager")
            .resizable(false)
            .exact_width(150.0)
            .show(ctx, |ui| {
                if let Some(idx) = self.po_index_focus {
                    let po = &mut self.purchase_orders[idx].order_status_ui(ui);
                };
            });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        ..Default::default()
    };
    let app = PoApp::default();

    eframe::run_native(
        "Purchasing Application",
        options,
        Box::new(|_cc| Box::new(app)),
    )
}
