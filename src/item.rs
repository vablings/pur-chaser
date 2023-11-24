use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, PartialEq)]
pub struct Item {
    pub qty: i32,
    pub part_number: String,
    pub description: String,
    pub unit_price: f64,
}
