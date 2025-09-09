use anyhow::Result;
use rust_xlsxwriter::{worksheet::Worksheet, ExcelDateTime, Format, FormatAlign};

pub fn write_order_date(
    worksheet: &mut Worksheet,
    row: u32,
    col: u16,
    order_date: f64,
    format: &Format,
) -> Result<()> {
    let excel_dt = ExcelDateTime::from_serial_datetime(order_date)?;
    worksheet.write_datetime_with_format(row, col, excel_dt, format)?;

    Ok(())
}

pub fn write_header_row(worksheet: &mut Worksheet, row: u32, format: &Format) -> Result<()> {
    let right_header = format.clone().set_align(FormatAlign::Right);

    worksheet.set_row_format(row, format)?;
    worksheet.write_row(row, 0, vec!["Date", "Employee", "Client", "Description"])?;
    worksheet.write_row_with_format(
        row,
        4,
        vec!["Count", "Hours", "Miles", "Grat"],
        &right_header,
    )?;
    worksheet.write_row(row, 8, vec!["Origin", "SubEvent", "Ready"])?;
    worksheet.write_string_with_format(row, 11, "Subtotal", &right_header)?;

    Ok(())
}
