use serde::Serialize;

use crate::process::deserialize::Order;

#[derive(Debug, Serialize)]
pub struct PreparedRow {
    pub order: Order,
    pub hours: f64,
    pub miles: f64,
}

#[derive(Debug, Serialize)]
pub struct ReferenceResult {
    pub rows: Vec<PreparedRow>,
    pub matched: u32,
    pub skipped: u32,
}
