use crate::process::{deserialize::TimeActivity, write::ExcelThemes};
use anyhow::Result;
use rust_xlsxwriter::worksheet::Worksheet;

pub fn write_unmatched_rows(
    worksheet: &mut Worksheet,
    timeactivities: &[TimeActivity],
    themes: &ExcelThemes,
) -> Result<()> {
    worksheet.set_column_width(0, 15)?;
    worksheet.set_column_width(1, 15)?;
    worksheet.set_column_width(2, 24)?;
    worksheet.set_column_width(3, 24)?;

    worksheet.write_row_with_format(
        0,
        0,
        vec!["First Name", "Last Name", "Clock In", "Clock Out"],
        &themes.header,
    )?;

    let mut row = 1;

    for entry in timeactivities.iter() {
        if entry.matched {
            continue;
        }

        worksheet.set_row_format(row, &themes.standard)?;
        worksheet.write_string_with_format(
            row,
            0,
            entry.first_name.to_string(),
            &themes.standard,
        )?;
        worksheet.write_string_with_format(
            row,
            1,
            entry.last_name.to_string(),
            &themes.standard,
        )?;
        worksheet.write_datetime_with_format(
            row,
            2,
            entry.in_time.naive_utc(),
            &themes.datetime,
        )?;
        worksheet.write_datetime_with_format(
            row,
            3,
            entry.out_time.naive_utc(),
            &themes.datetime,
        )?;

        row += 1;
    }

    Ok(())
}
