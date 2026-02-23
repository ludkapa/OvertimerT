use crate::excel::cells_constants::*;
use crate::{
    entities::day::{Day, DayType, Season},
    excel::styles::{CellType, DataType, cell_style},
};
use anyhow::Result as AResult;
use rust_xlsxwriter::{FormatBorder, Formula, worksheet::Worksheet};

pub(super) fn add_day_cell(month_worksheet: &mut Worksheet, day: &Day) -> AResult<DayType> {
    let day_row = ROW_DAY_OFFSET + day.number();
    // Types for day marking
    let day_type_format = match day.earn_type() {
        DayType::Earn => cell_style(DataType::UsualText, CellType::Earn),
        DayType::Weekend => cell_style(DataType::UsualText, CellType::Weekend),
        DayType::Usual => cell_style(DataType::UsualText, CellType::Usual),
    };
    // Day/Weekday
    month_worksheet.write_with_format(
        day_row,
        COL_DATE,
        format!("{} {}", day.number(), day.weekday_short()),
        &day_type_format
            .clone()
            .set_border_left(FormatBorder::Medium),
    )?;
    // Overworked hours
    month_worksheet.write_with_format(day_row, COL_OVERWORKED_COUNTER, "0", &day_type_format)?;
    // Bonus formula
    let bonus_formula_format = cell_style(DataType::Money, CellType::TotalBonus);
    month_worksheet.write_formula_with_format(
        day_row,
        COL_BONUS,
        Formula::new(format!("=E5/E1*B{}*2", day_row + 1)),
        &bonus_formula_format,
    )?;
    Ok(day.earn_type())
}

pub(super) fn add_header_cells(month_worksheet: &mut Worksheet, first_day: &Day) -> AResult<()> {
    // Year
    let mut format = cell_style(DataType::UsualText, CellType::Header);
    month_worksheet.write_with_format(ROW_YEAR, COL_YEAR, first_day.year(), &format)?;
    // AppInfo
    month_worksheet.merge_range(
        ROW_INFO_START,
        COL_INFO_START,
        ROW_INFO_END,
        COL_INFO_END,
        "dev.release",
        &format,
    )?;
    // Day header
    month_worksheet.write_with_format(ROW_DAYS_HEADER, COL_DATE, "Число/День", &format)?;
    // Bonus header
    month_worksheet.write_with_format(
        ROW_DAYS_HEADER,
        COL_OVERWORKED_COUNTER,
        "Доплата",
        &format,
    )?;
    format = cell_style(DataType::UsualText, CellType::InputHeader);
    // Hours header
    month_worksheet.write_with_format(ROW_DAYS_HEADER, COL_BONUS, "Часы", &format)?;
    // Month
    format = match first_day.season() {
        Season::Winter => cell_style(DataType::UsualText, CellType::MonthWinter),
        Season::Spring => cell_style(DataType::UsualText, CellType::MonthSpring),
        Season::Summer => cell_style(DataType::UsualText, CellType::MonthSummer),
        Season::Autumn => cell_style(DataType::UsualText, CellType::MonthAutumn),
    };
    month_worksheet.merge_range(
        ROW_MONTH_START,
        COL_MONTH_START,
        ROW_MONTH_END,
        COL_MONTH_END,
        first_day.month_name().as_str(),
        &format,
    )?;
    Ok(())
}

pub(super) fn add_salary(month_worksheet: &mut Worksheet, salary: u32) -> AResult<()> {
    // Salary input header
    let header_format = cell_style(DataType::UsualText, CellType::InputHeader);
    month_worksheet.write_with_format(ROW_SALARY, COL_SALARY_HEADER, "Оклад:", &header_format)?;
    // Salary
    let input_format = cell_style(DataType::Money, CellType::InputHeader);
    month_worksheet.write_formula_with_format(
        ROW_SALARY,
        COL_SALARY,
        format!("={}", salary).as_str(),
        &input_format,
    )?;
    Ok(())
}

pub(super) fn add_total_hours(month_worksheet: &mut Worksheet, work_hours: u16) -> AResult<()> {
    let format = cell_style(DataType::UsualText, CellType::Header);
    // Total month hours header
    month_worksheet.write_with_format(
        ROW_TOTAL_HOURS,
        COL_TOTAL_HOURS_HEADER,
        "Рабочие часы:",
        &format,
    )?;
    // Hours
    month_worksheet.write_with_format(ROW_TOTAL_HOURS, COL_TOTAL_HOURS, work_hours, &format)?;
    Ok(())
}

pub(super) fn add_weekend_hours(
    month_worksheet: &mut Worksheet,
    weekends_formula: String,
) -> AResult<()> {
    let format = cell_style(DataType::UsualText, CellType::Header);
    // Total weekends hours header
    month_worksheet.write_with_format(
        ROW_WEEKEND_HOURS,
        COL_WEEKEND_HOURS_HEADER,
        "Часы выходных:",
        &format,
    )?;
    // Weekend hours formula
    month_worksheet.write_formula_with_format(
        ROW_WEEKEND_HOURS,
        COL_WEEKEND_HOURS,
        Formula::new(weekends_formula),
        &format,
    )?;
    Ok(())
}

pub(super) fn add_overworked_hours(
    month_worksheet: &mut Worksheet,
    usual_days_formula: String,
) -> AResult<()> {
    let format = cell_style(DataType::UsualText, CellType::Header);
    // Total overwork hours header
    month_worksheet.write_with_format(
        ROW_OVERWORKED_HOURS,
        COL_OVERWORKED_HOURS_HEADER,
        "Часы переработки:",
        &format,
    )?;
    // Overtime hours formula
    month_worksheet.write_formula_with_format(
        ROW_OVERWORKED_HOURS,
        COL_OVERWORKED_HOURS,
        Formula::new(usual_days_formula),
        &format,
    )?;
    Ok(())
}

pub(super) fn add_total_payment(month_worksheet: &mut Worksheet, total_days: u8) -> AResult<()> {
    // Total payout header
    let header_format = cell_style(DataType::UsualText, CellType::TotalPayment);
    month_worksheet.write_with_format(
        ROW_TOTAL_PAYMENT,
        COL_TOTAL_PAYMENT_HEADER,
        "К получению:",
        &header_format,
    )?;
    // Total payment formula
    let formula_format = cell_style(DataType::Money, CellType::TotalPayment);
    month_worksheet.write_formula_with_format(
        ROW_TOTAL_PAYMENT,
        COL_TOTAL_PAYMENT,
        Formula::new(format!("=SUM(C4:C{})+E5", total_days + 4)),
        &formula_format,
    )?;
    Ok(())
}

pub(super) fn add_bottom_border(
    month_worksheet: &mut Worksheet,
    bottom_of_days: u32,
) -> AResult<()> {
    let format = cell_style(DataType::UsualText, CellType::BottomBorder);
    month_worksheet.merge_range(
        bottom_of_days,
        COL_DATE,
        bottom_of_days,
        COL_BONUS,
        "",
        &format,
    )?;
    Ok(())
}
