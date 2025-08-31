use crate::process::{
    compare::{cross_reference_orders, PreparedRow},
    deserialize::{Order, TimeActivity},
    expand::expand_orders,
    write::write_new_xlsx,
};
use anyhow::Result;

mod compare;
pub mod deserialize;
mod expand;
pub mod validate;
mod write;

pub fn process(caterease: &[Order], intuit: &mut [TimeActivity]) -> Result<usize> {
    let expanded = expand_orders(caterease);

    let prepared_rows: Vec<PreparedRow> = cross_reference_orders(&expanded, intuit);

    let to_write = prepared_rows.len();

    write_new_xlsx(prepared_rows)?;

    Ok(to_write)
}
