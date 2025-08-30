use crate::process::deserialize::Order;

#[derive(Debug)]
pub struct PreparedRow {
    pub order: Order,
    pub hours: f64,
    pub miles: f64,
}
