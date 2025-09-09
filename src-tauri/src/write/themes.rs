use rust_xlsxwriter::{Color, Format, FormatAlign, FormatBorder};

const LT_GRAY: u32 = 0xE5E7EB;
const PASTEL_YELLOW: u32 = 0xFFFFBA;

pub struct ExcelThemes {
    pub standard: Format,
    pub expanded: Format,
    pub header: Format,
    pub money: Format,
    pub right_align: Format,
    pub date: Format,
    pub time: Format,
    pub datetime: Format,
}

pub fn build_themes() -> ExcelThemes {
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
    let time = standard.clone().set_num_format("hh:mm AM/PM");
    let datetime = standard.clone().set_num_format("YYYY-MM-DD HH:MM:SS");

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
