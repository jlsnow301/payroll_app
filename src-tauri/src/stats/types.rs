use serde::Serialize;

#[derive(Debug, Default)]
pub struct DriverAccumulator {
    pub total_diff_seconds: i64,
    pub count: u32,
    pub late_count: u32,
    pub max_late_diff_seconds: i64, // largest single positive diff (latest clock-in)
}

#[derive(Serialize)]
pub struct DriverStats {
    /// Most frequently used driver
    pub top_used: String,
    /// Count of uses for most frequently used driver
    pub top_used_count: u32,
    /// Most punctual driver (fewest late, then lowest avg diff)
    pub punctual: String,
    /// Average clock-in difference in minutes (signed: positive = late, negative = early)
    pub punctual_avg: f64,
    /// Total late clock-ins for punctual driver
    pub punctual_late_count: u32,
    /// Driver with most late clock-ins (raw count)
    pub most_late: String,
    /// Count of late clock-ins for most late driver
    pub most_late_count: u32,
    /// Driver with highest percentage of late clock-ins
    pub highest_late_percent_driver: String,
    /// Highest late percentage (0-100)
    pub highest_late_percent: f64,
    /// Driver with the single latest clock-in (max positive diff)
    pub latest_clock_in_driver: String,
    /// Latest clock-in diff in minutes (positive = minutes late)
    pub latest_clock_in_diff_minutes: f64,
}
