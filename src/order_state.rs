use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, Default)]

pub struct Signature {
    approved_by: String,
}
impl Signature {
    fn sign() -> Self {
        todo!();
    }
    pub fn none() -> Self {
        Self {
            approved_by: "".to_string(),
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Default)]
pub enum OrderStatus {
    #[default]
    Unapproved,
    Approved(Signature),
    Ordered(Signature),
    Recieved(Signature),
    Paid(Signature),
    OrderComplete,
}
impl Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unapproved => write!(f, "Unapproved"),
            Self::Approved(T) => write!(f, "Approved"),
            Self::Ordered(T) => write!(f, "Ordered"),
            Self::Recieved(T) => write!(f, "Recieved"),
            Self::Paid(T) => write!(f, "Paid"),
            Self::OrderComplete => write!(f, "Completed"),
        }
    }
}

impl OrderStatus {
    fn approve(&mut self, signature: Signature) {
        match self {
            OrderStatus::Unapproved => *self = OrderStatus::Approved(signature),
            _ => {}
        }
    }
}

/*


type Signature = String;
#[derive(Default, Serialize, Deserialize, PartialEq, Clone, Debug)]enum StatusType {
    #[default]
    PendingApproval,
    ApprovedToOrder(Signature),
    OrderRaised(Signature),
    GoodsRecieved(Signature),
    InvoicePaid(Signature),
    OrderComplete,
}

impl fmt::Display for StatusType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::PendingApproval => write!(f, "Pending Approval"),
            Self::ApprovedToOrder(S) => write!(f, "Awating Ordering"),
            Self::OrderRaised(S) => write!(f, "Awating Delivery"),
            Self::GoodsRecieved(S) => write!(f, "Payment Required"),
            Self::InvoicePaid(S) => write!(f, "Order Complete"),
            Self::OrderComplete => write!(f, "Order Complete"),
        }
    }
}
#[derive(Default, Serialize, Deserialize, PartialEq)]
struct Status {
    current_status: StatusType,
    status_history: Vec<StatusType>,
}
impl Status {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Current status");
        ui.monospace(format!("{}", self.current_status));
        let mut username: String = whoami::username();
        match &self.current_status {
            StatusType::PendingApproval => {
                if ui.button("Approve Purchase").clicked() {
                    self.current_status = StatusType::ApprovedToOrder(username.clone());
                    self.status_history.push(self.current_status.clone());
                };
            }
            StatusType::ApprovedToOrder(T) => {
                if ui.button("Confirm Order").clicked() {
                    self.current_status = StatusType::OrderRaised(username.clone());
                    self.status_history.push(self.current_status.clone());
                };
            }
            StatusType::OrderRaised(T) => {
                if ui.button("Confirm Goods Recieved").clicked() {
                    self.current_status = StatusType::GoodsRecieved(username.clone());
                    self.status_history.push(self.current_status.clone());
                };
            }
            StatusType::GoodsRecieved(T) => {
                if ui.button("Confirm invoice paid").clicked() {
                    self.current_status = StatusType::InvoicePaid(username.clone());
                    self.status_history.push(self.current_status.clone());
                };
            }
            StatusType::GoodsRecieved(T) => {
                if ui.button("Confirm invoice paid").clicked() {
                    self.current_status = StatusType::InvoicePaid(username.clone());
                    self.status_history.push(self.current_status.clone());
                };
            }
            StatusType::OrderComplete => {
                ui.label("order complete");
            }
            _ => {}
        }
        if self.current_status != StatusType::OrderComplete {
            ui.text_edit_singleline(&mut username);
        }

        ui.separator();
        ui.heading("Operation History");
        egui::ScrollArea::vertical().show(ui, |ui| {
            for record in &mut self.status_history {
                ui.label(format!("{:?}", record));
            }
        });
    }
}
    */
