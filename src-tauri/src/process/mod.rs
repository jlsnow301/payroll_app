use crate::process::{
    compare::cross_reference_orders,
    deserialize::{Order, TimeActivity},
    expand::expand_orders,
    write::write_new_xlsx,
};
use anyhow::Result;
use serde::Serialize;

mod compare;
pub mod deserialize;
mod expand;
pub mod validate;
mod write;

#[derive(Serialize)]
pub struct ProcessResult {
    pub expanded: usize,
    pub matched: u32,
    pub total: usize,
    pub missing: u32,
}

pub fn process(
    caterease: &[Order],
    intuit: &mut [TimeActivity],
    precision: i64,
) -> Result<ProcessResult> {
    let expanded = expand_orders(caterease);

    let reference_result = cross_reference_orders(&expanded, intuit, precision);

    write_new_xlsx(&reference_result.rows)?;

    Ok(ProcessResult {
        expanded: expanded.len() - caterease.len(),
        matched: reference_result.matched,
        total: expanded.len(),
        missing: intuit.len() as u32 - reference_result.matched,
    })
}
