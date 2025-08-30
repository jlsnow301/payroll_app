use crate::process::{
    check::test_order_input,
    compare::{cross_reference_orders, PreparedRow},
    deserialize::{deserialize_caterease_excel, deserialize_intuit_excel, Order, TimeActivity},
    expand::expand_orders,
    write::write_new_xlsx,
};
use anyhow::Result;

mod check;
mod compare;
mod deserialize;
mod expand;
mod write;

pub fn process(caterease: &str, intuit: &str) -> Result<usize> {
    let orders: Vec<Order> = deserialize_caterease_excel(caterease)?;

    test_order_input(&orders)?;

    let expanded = expand_orders(&orders);

    let mut timesheets: Vec<TimeActivity> = deserialize_intuit_excel(intuit)?;

    let prepared_rows: Vec<PreparedRow> = cross_reference_orders(&expanded, &mut timesheets);

    let to_write = prepared_rows.len();

    write_new_xlsx(prepared_rows)?;

    Ok(to_write)
}
