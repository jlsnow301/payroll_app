use crate::{
    process::compare::PreparedRow,
    process::write::util::{write_header_row, write_order_date},
};
use anyhow::Result;
use rust_xlsxwriter::{workbook::Workbook, Color, Format, FormatAlign, FormatBorder};

const LT_GRAY: u32 = 0xE5E7EB;

pub fn write_new_xlsx(rows: Vec<PreparedRow>) -> Result<()> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

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

    let mut row = 0;

    // Create themes
    let standard = Format::new()
        .set_border(FormatBorder::Thin)
        .set_border_color(Color::Gray);
    let header = standard.clone().set_background_color(Color::RGB(LT_GRAY));
    let money = standard
        .clone()
        .set_align(FormatAlign::Right)
        .set_num_format("[$$-409]#,##0.0");
    let right_align = standard.clone().set_align(FormatAlign::Right);
    let date = standard.clone().set_num_format("mm/dd/yyyy");
    let time = standard.clone().set_num_format("hh:mm AM/PM");

    write_header_row(worksheet, row, &header)?;
    row += 1;

    for entry in rows.iter() {
        worksheet.set_row_format(row, &standard)?;
        write_order_date(worksheet, row, 0, entry.order.date, &date)?;
        worksheet.write_string(row, 1, entry.order.employee.to_string())?;
        worksheet.write_string(row, 2, entry.order.client.to_string())?;
        worksheet.write_string(row, 3, entry.order.description.to_string())?;
        worksheet.write_number_with_format(row, 4, entry.order.count as f64, &right_align)?;
        worksheet.write_number_with_format(row, 5, entry.hours, &right_align)?;
        worksheet.write_number_with_format(row, 6, entry.miles, &right_align)?;
        worksheet.write_number_with_format(row, 7, entry.order.grat, &money)?;
        worksheet.write_string(row, 8, entry.order.origin.to_string())?;
        worksheet.write_string(row, 9, entry.order.event.to_string())?;
        write_order_date(worksheet, row, 10, entry.order.ready, &time)?;
        worksheet.write_number_with_format(row, 11, entry.order.total, &money)?;

        row += 1;
    }

    workbook.save("../formatted_payroll.xlsx")?;
    Ok(())
}
