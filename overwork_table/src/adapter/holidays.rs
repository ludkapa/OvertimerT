use anyhow::Result as AResult;
use chrono::Datelike;
use chrono::NaiveDate;
use serde::Deserialize;
use std::collections::HashSet;

const HOLIDAYS_URL: &str = "https://raw.githubusercontent.com/d10xa/holidays-calendar/refs/heads/master/json/calendar.json";

#[derive(Deserialize, Debug, PartialEq)]
pub struct HolidayDates {
    holidays: HashSet<NaiveDate>,
}

impl HolidayDates {
    pub async fn init() -> AResult<Self> {
        // Url with holidays array in "year-month-day" format
        let url = HOLIDAYS_URL;
        // Fetch data
        let response = reqwest::get(url).await?.error_for_status()?;
        // Deserialize responce body into FetchedDates struct
        let mut fetched_dates = response.json::<HolidayDates>().await?;
        // It will be optimized to current year fetching
        // Try to find last year
        let last_year = match fetched_dates.holidays.iter().map(|d| d.year()).max() {
            Some(year) => year,
            None => return Err(anyhow::anyhow!("Cannot find last year!")),
        };
        // Get all holidays dates at finded year
        fetched_dates.holidays = fetched_dates
            .holidays
            .into_iter()
            .filter(|d| d.year() == last_year)
            .collect();
        // Return
        Ok(fetched_dates)
    }

    pub(crate) fn get_holidays(&self) -> &HashSet<NaiveDate> {
        &self.holidays
    }
}
