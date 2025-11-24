use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::deserialize::Order;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreparedRow {
    pub order: Order,
    pub hours: f64,
    pub miles: f64,
    pub suggested_in: Option<DateTime<Utc>>,
    pub suggested_out: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct ReferenceResult {
    pub rows: Vec<PreparedRow>,
    /// Number of matched entries   
    pub matched: u32,
    /// Invalid orders etc
    pub skipped: u32,
}
