use std::sync::LazyLock;

pub static CATEREASE_HEADERS: LazyLock<Vec<String>> = LazyLock::new(|| {
    vec![
        "Date".into(),
        "Delivery Person".into(),
        "Client/Organization".into(),
        "Description".into(),
        "Actual".into(),
        "Grat".into(),
        "Delivery Category".into(),
        "Sub-Event #".into(),
        "Kitchen Ready by".into(),
        "Subtotal".into(),
    ]
});

pub static INTUIT_HEADERS: LazyLock<Vec<String>> = LazyLock::new(|| {
    vec![
        "First name".into(),
        "Last name".into(),
        "Username".into(),
        "Start time".into(),
        "End time".into(),
        "Customer".into(),
        "Hours".into(),
        "Miles".into(),
    ]
});
