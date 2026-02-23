use crate::{
    adapter::holidays::HolidayDates,
    entities::{
        day::{Day, DayType},
        days::DaysBuilder,
        generate_params::GenerateParams,
    },
    excel::{
        cells_constants::{COL_BONUS, COL_OVERWORKED_COUNTER, COL_TOTAL_HOURS, ROW_DAY_OFFSET},
        cells_filling::{
            add_bottom_border, add_day_cell, add_header_cells, add_overworked_hours, add_salary,
            add_total_hours, add_total_payment, add_weekend_hours,
        },
    },
};
use anyhow::Result as AResult;
use rust_xlsxwriter::{workbook::Workbook, worksheet::Worksheet};

pub async fn get_filled_table(gen_params: GenerateParams) -> AResult<Vec<u8>> {
    // Fetch holidays
    let holidays = HolidayDates::init().await?;
    // Generate days for filling
    let days = match gen_params.shift() {
        Some(shift) => DaysBuilder::new()
            .with_holidays(holidays.get_holidays())
            .with_shift_type(shift)
            .build(),
        None => DaysBuilder::new()
            .with_holidays(holidays.get_holidays())
            .build(),
    };

    // Split days to chunks by month
    let chunks = days.split_months();

    // Creating table
    let mut table = Workbook::new();

    for month_days in chunks {
        build_month_sheet(&mut table, month_days, &gen_params)?;
    }
    // Convert struct to bytes and return it
    let buf = table.save_to_buffer()?;
    Ok(buf)
}

fn build_month_sheet(
    table: &mut Workbook,
    month_days: &[Day],
    gen_params: &GenerateParams,
) -> AResult<()> {
    // Creating Sheet
    let month_worksheet = table.add_worksheet();
    // Set month name
    month_worksheet.set_name(month_days.first().unwrap().month_name())?;

    // For total formulas
    let mut weekend_cells: Vec<String> = Vec::new();
    let mut usual_day_cells: Vec<String> = Vec::new();
    // For work hours
    let mut work_hours: u16 = 0;
    // Iterate over days in month chunk
    for day in month_days {
        build_day(
            month_worksheet,
            day,
            &mut weekend_cells,
            &mut usual_day_cells,
            &mut work_hours,
        )?;
    }

    // Adding headers
    add_header_cells(month_worksheet, month_days.first().unwrap())?;

    // Make formula string
    let weekends_formula = format!("={}", weekend_cells.join("+"));
    let usual_days_formula = format!("={}", usual_day_cells.join("+"));

    // Add a total block
    add_total_hours(month_worksheet, work_hours)?;
    add_weekend_hours(month_worksheet, weekends_formula)?;
    add_overworked_hours(month_worksheet, usual_days_formula)?;
    add_total_payment(month_worksheet, month_days.len() as u8)?;
    add_salary(month_worksheet, gen_params.salary())?;

    // Final styles
    polish_worksheet(month_worksheet)?;

    // Add bottom border
    let bottom_of_days = month_days.len() as u32 + ROW_DAY_OFFSET as u32 + 1;
    add_bottom_border(month_worksheet, bottom_of_days)?;
    Ok(())
}

fn build_day(
    month_worksheet: &mut Worksheet,
    day: &Day,
    weekend_cells: &mut Vec<String>,
    usual_day_cells: &mut Vec<String>,
    work_hours: &mut u16,
) -> AResult<()> {
    // Row number
    let row_number = 3 + day.number();

    // Adding day to month sheet and geting this flag
    let flag = add_day_cell(month_worksheet, day)?;

    // Creating formula for total block
    match flag {
        DayType::Usual => {
            usual_day_cells.push(format!("B{}", row_number));
            *work_hours = *work_hours + 8;
        }
        _ => {
            weekend_cells.push(format!("B{}", row_number));
        }
    }
    Ok(())
}

fn polish_worksheet(month_worksheet: &mut Worksheet) -> AResult<()> {
    // Autofit columns
    month_worksheet.autofit();
    // Make worksheet white
    month_worksheet.set_screen_gridlines(false);
    // Make E column wider
    month_worksheet.set_column_width(COL_TOTAL_HOURS, 12)?;
    // Make C column wider
    month_worksheet.set_column_width(COL_BONUS, 10)?;
    // Make B column narrower
    month_worksheet.set_column_width(COL_OVERWORKED_COUNTER, 7.5)?;
    Ok(())
}

mod cells_constants;
mod cells_filling;
mod styles;
mod styles_constants;
