use std::{path::PathBuf};

use crate::{
    compare::PreparedRow,
    deserialize::TimeActivity,
    write::util::{write_header_row, write_order_date, write_order_timestamp},
};
use anyhow::{Context, Error, Result};
use rust_xlsxwriter::{workbook::Workbook, Color, Format, FormatAlign, FormatBorder};

const LT_GRAY: u32 = 0xE5E7EB;
const PASTEL_YELLOW: u32 = 0xFFFFBA;

struct ExcelThemes {
    pub standard: Format,
    pub expanded: Format,
    pub header: Format,
    pub money: Format,
    pub right_align: Format,
    pub date: Format,
    pub time: Format,
    pub datetime: Format,
}

pub struct WorkbookWriter {
    workbook: Workbook,
    themes: ExcelThemes,
}

impl WorkbookWriter {
    pub fn new() -> Self {
        Self {
            workbook: Workbook::new(),
            themes: Self::build_themes(),
        }
    }

    fn build_themes() -> ExcelThemes {
        let standard = Format::new()
            .set_border(FormatBorder::Thin)
            .set_border_color(Color::Gray);
        let expanded = standard
            .clone()
            .set_background_color(Color::RGB(PASTEL_YELLOW));
        let header = standard.clone().set_background_color(Color::RGB(LT_GRAY));
        let money = standard
            .clone()
            .set_align(FormatAlign::Right)
            .set_num_format("[$$-409]#,##0.0");
        let right_align = standard.clone().set_align(FormatAlign::Right);
        let date = standard.clone().set_num_format("mm/dd/yyyy");
        let time = standard.clone().set_num_format("h:mm AM/PM");
        let datetime = standard.clone().set_num_format("YYYY-MM-DD h:mm AM/PM");

        ExcelThemes {
            standard,
            expanded,
            header,
            money,
            right_align,
            date,
            time,
            datetime,
        }
    }

    pub fn write_prepared(&mut self, rows: &[PreparedRow]) -> Result<(), Error> {
        let worksheet = self
            .workbook
            .add_worksheet()
            .set_name("Orders")
            .context("Couldn't add orders to worksheet")?;

        // write_prepared_rows(orders_sheet, rows, &self.themes)

        // Default sizing
        worksheet.set_column_width(0, 12)?;
        worksheet.set_column_width(1, 24)?;
        worksheet.set_column_width(2, 48)?;
        worksheet.set_column_width(3, 36)?;
        // count, hours, miles, grat ok
        worksheet.set_column_range_width(4, 13, 12)?;
        write_header_row(worksheet, 0, &self.themes.header)?;

        let mut row = 1;

        for entry in rows.iter() {
            let to_use = match entry.order.expanded {
                true => &self.themes.expanded,
                false => &self.themes.standard,
            };

            worksheet.set_row_format(row, &self.themes.standard)?;
            write_order_date(worksheet, row, 0, entry.order.date, &self.themes.date)?;
            worksheet.write_string_with_format(row, 1, entry.order.employee.to_string(), to_use)?;
            worksheet.write_string_with_format(row, 2, entry.order.client.to_string(), to_use)?;
            worksheet.write_string(row, 3, entry.order.description.to_string())?;
            worksheet.write_number_with_format(
                row,
                4,
                entry.order.count as f64,
                &self.themes.right_align,
            )?;
            worksheet.write_number_with_format(row, 5, entry.hours, &self.themes.right_align)?;
            worksheet.write_number_with_format(row, 6, entry.miles, &self.themes.right_align)?;
            worksheet.write_number_with_format(row, 7, entry.order.grat, &self.themes.money)?;
            worksheet.write_string(row, 8, entry.order.origin.to_string())?;
            worksheet.write_string(row, 9, entry.order.event.to_string())?;
            write_order_date(worksheet, row, 10, entry.order.ready, &self.themes.time)?;
            worksheet.write_number_with_format(row, 11, entry.order.total, &self.themes.money)?;

            if let (Some(clock_in), Some(clock_out)) = (entry.suggested_in, entry.suggested_out) {
                write_order_timestamp(worksheet, row, 12, clock_in, &self.themes.time)?;
                write_order_timestamp(worksheet, row, 13, clock_out, &self.themes.time)?;
            }
            row += 1;
        }

        Ok(())
    }

    pub fn write_unmatched(&mut self, rows: &[TimeActivity]) -> Result<(), Error> {
        let worksheet = self
            .workbook
            .add_worksheet()
            .set_name("Unmatched")
            .context("Couldn't add unmatched sheet")?;

        worksheet.set_column_width(0, 15)?;
        worksheet.set_column_width(1, 15)?;
        worksheet.set_column_width(2, 24)?;
        worksheet.set_column_width(3, 24)?;

        worksheet.write_row_with_format(
            0,
            0,
            vec!["First Name", "Last Name", "Clock In", "Clock Out"],
            &self.themes.header,
        )?;

        let mut row = 1;

        for entry in rows.iter() {
            if entry.matched {
                continue;
            }

            worksheet.set_row_format(row, &self.themes.standard)?;
            worksheet.write_string_with_format(
                row,
                0,
                entry.first_name.to_string(),
                &self.themes.standard,
            )?;
            worksheet.write_string_with_format(
                row,
                1,
                entry.last_name.to_string(),
                &self.themes.standard,
            )?;
            write_order_timestamp(worksheet, row, 2, entry.in_time, &self.themes.datetime)?;
            write_order_timestamp(worksheet, row, 3, entry.out_time, &self.themes.datetime)?;

            row += 1;
        }

        Ok(())
    }

    pub fn save(&mut self, path: PathBuf) -> Result<(), anyhow::Error> {        
        self.workbook
            .save(&path)
            .context("Couldn't save workbook")
    }
}
