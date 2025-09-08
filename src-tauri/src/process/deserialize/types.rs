use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Order {
    pub date: f64,
    pub employee: String,
    pub client: String,
    pub description: String,
    pub count: i64,
    pub grat: f64,
    pub origin: String,
    pub event: String,
    pub ready: f64,
    pub total: f64,
    // Joined date/time of the event
    pub datetime: DateTime<Utc>,
    // Order has been expanded for having multiple drivers
    pub expanded: bool,
    pub nearest: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize)]
pub struct TimeActivity {
    pub first_name: String,
    pub last_name: String,
    // pub full_name: String,
    pub in_time: DateTime<Utc>,
    pub out_time: DateTime<Utc>,
    pub hours: f64,
    pub miles: f64,
    // This has already been matched with an event
    pub matched: bool,
}
