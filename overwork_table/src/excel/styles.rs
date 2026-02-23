use crate::excel::styles_constants::*;
use rust_xlsxwriter::{Color, Format, FormatBorder};

pub(crate) enum DataType {
    Money,     // Rounded to 2 decimal places and formatted as currency
    UsualText, // Nothing to do
}

pub(crate) enum CellType {
    Usual,        // For usual cells - white background dotted border and bold font
    Night,        // For night shift cells - gray background dotted border and bold font
    Weekend,      // For weekend day cells - white red background dotted border and bold font
    Earn,         // For earn cells - green background dotted border and bold font
    NightEarn,    // For night earn cells - black-green background dotted border and bold font
    Header,       // For header cells - pink background and solid border normal font
    TotalBonus,   // For total bonus cells - white background solid border and bold font
    TotalPayment, // For earn cells - green background solid border and bold font
    InputHeader,  // For input header cells - orange background solid border and normal font
    MonthWinter,  // For month winter cells - blue background solid border and normal font
    MonthSummer,  // For month summer cells - yellow background solid border and normal font
    MonthAutumn,  // For month autumn cells - orange background solid border and normal font
    MonthSpring,  // For month spring cells - green background solid border and normal font
    BottomBorder, // For solid border that will pasted after all days
}

pub(crate) fn cell_style(data_type: DataType, cell_type: CellType) -> Format {
    let mut format = match cell_type {
        CellType::Usual => Format::new().set_border(FormatBorder::Dotted).set_bold(),

        CellType::Night => Format::new()
            .set_border(FormatBorder::Dotted)
            .set_bold()
            .set_background_color(BG_NIGHT),

        CellType::Weekend => Format::new()
            .set_border(FormatBorder::Dotted)
            .set_bold()
            .set_background_color(Color::RGB(BG_WEEKEND)),

        CellType::Earn => Format::new()
            .set_border(FormatBorder::Dotted)
            .set_bold()
            .set_background_color(Color::RGB(BG_EARN)),

        CellType::NightEarn => Format::new()
            .set_border(FormatBorder::Dotted)
            .set_bold()
            .set_background_color(Color::RGB(BG_NIGHT_EARN)),

        CellType::Header => Format::new()
            .set_border(FormatBorder::Medium)
            .set_background_color(Color::RGB(BG_HEADER)),

        CellType::TotalBonus => Format::new().set_border(FormatBorder::Medium).set_bold(),

        CellType::TotalPayment => Format::new()
            .set_border(FormatBorder::Medium)
            .set_bold()
            .set_background_color(Color::RGB(BG_TOTAL_PAYMENT)),

        CellType::InputHeader => Format::new()
            .set_border(FormatBorder::Medium)
            .set_background_color(Color::RGB(BG_INPUT_HEADER)),

        CellType::MonthWinter => Format::new()
            .set_border(FormatBorder::Medium)
            .set_background_color(Color::RGB(BG_MONTH_WINTER)),

        CellType::MonthSpring => Format::new()
            .set_border(FormatBorder::Medium)
            .set_background_color(Color::RGB(BG_MONTH_SPRING)),

        CellType::MonthSummer => Format::new()
            .set_border(FormatBorder::Medium)
            .set_background_color(Color::RGB(BG_MONTH_SUMMER)),

        CellType::MonthAutumn => Format::new()
            .set_border(FormatBorder::Medium)
            .set_background_color(Color::RGB(BG_MONTH_AUTUMN)),

        CellType::BottomBorder => Format::new().set_border_top(FormatBorder::Medium),
    };

    format = match data_type {
        DataType::Money => format.set_num_format(FORMAT_MONEY),
        DataType::UsualText => format,
    };

    format.set_align(rust_xlsxwriter::FormatAlign::Center)
}
