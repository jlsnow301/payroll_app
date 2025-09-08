use crate::process::{
    compare::PreparedRow,
    write::{
        util::{write_header_row, write_order_date},
        ExcelThemes,
    },
};
use anyhow::Result;
use rust_xlsxwriter::worksheet::Worksheet;

pub fn write_prepared_rows(
    worksheet: &mut Worksheet,
    rows: &[PreparedRow],
    themes: &ExcelThemes,
) -> Result<()> {
    // Default sizing
    worksheet.set_column_width(0, 12)?;
    worksheet.set_column_width(1, 24)?;
    worksheet.set_column_width(2, 48)?;
    worksheet.set_column_width(3, 36)?;
    // count, hours, miles, grat ok
    worksheet.set_column_width(7, 12)?;
    worksheet.set_column_width(8, 12)?;
    worksheet.set_column_width(9, 12)?;
    worksheet.set_column_width(10, 12)?;
    worksheet.set_column_width(11, 12)?;
    write_header_row(worksheet, 0, &themes.header)?;

    let mut row = 1;

    for entry in rows.iter() {
        let to_use = match entry.order.expanded {
            true => &themes.expanded,
            false => &themes.standard,
        };

        worksheet.set_row_format(row, &themes.standard)?;
        write_order_date(worksheet, row, 0, entry.order.date, &themes.date)?;
        worksheet.write_string_with_format(row, 1, entry.order.employee.to_string(), to_use)?;
        worksheet.write_string_with_format(row, 2, entry.order.client.to_string(), to_use)?;
        worksheet.write_string(row, 3, entry.order.description.to_string())?;
        worksheet.write_number_with_format(
            row,
            4,
            entry.order.count as f64,
            &themes.right_align,
        )?;
        worksheet.write_number_with_format(row, 5, entry.hours, &themes.right_align)?;
        worksheet.write_number_with_format(row, 6, entry.miles, &themes.right_align)?;
        worksheet.write_number_with_format(row, 7, entry.order.grat, &themes.money)?;
        worksheet.write_string(row, 8, entry.order.origin.to_string())?;
        worksheet.write_string(row, 9, entry.order.event.to_string())?;
        write_order_date(worksheet, row, 10, entry.order.ready, &themes.time)?;
        worksheet.write_number_with_format(row, 11, entry.order.total, &themes.money)?;

        row += 1;
    }

    Ok(())
}
