use crate::{
    process::{
        compare::{cross_reference_orders, ReferenceResult},
        deserialize::{Order, TimeActivity},
        expand::expand_orders,
        write::{build_themes, write_prepared_rows, write_unmatched_rows},
    },
    util::OUTPUT_PATH,
};

use anyhow::{Context, Result};
use rust_xlsxwriter::workbook::Workbook;

pub fn get_references(
    caterease: &mut [Order],
    intuit: &mut [TimeActivity],
    precision: i64,
) -> Result<ReferenceResult> {
    let mut expanded = expand_orders(caterease);

    let reference_result = cross_reference_orders(&mut expanded, intuit, precision);

    Ok(reference_result)
}

pub fn write_excel(referenced: &ReferenceResult, intuit: &[TimeActivity]) -> Result<()> {
    let mut workbook = Workbook::new();
    let orders_sheet = workbook
        .add_worksheet()
        .set_name("Orders")
        .context("Couldn't add orders worksheet")?;

    let themes = build_themes();

    write_prepared_rows(orders_sheet, &referenced.rows, &themes)?;

    let unmatched_sheet = workbook
        .add_worksheet()
        .set_name("Unmatched")
        .context("Couldn't add unmatched sheet")?;

    write_unmatched_rows(unmatched_sheet, intuit, &themes)?;

    workbook
        .save(OUTPUT_PATH)
        .context("Couldn't save workbook")?;

    Ok(())
}
