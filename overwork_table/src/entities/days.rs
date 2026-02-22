use chrono::{Datelike, Local, NaiveDate, Weekday};
use derive_more::{Deref, DerefMut, IntoIterator};
use std::collections::HashSet;

use crate::entities::{
    day::{Day, DayType},
    generate_params::WorkShift,
};

#[derive(IntoIterator, Deref, DerefMut)]
pub(crate) struct Days(Vec<Day>);

impl FromIterator<Day> for Days {
    fn from_iter<T: IntoIterator<Item = Day>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Days {
    pub(crate) fn split_months(&self) -> impl Iterator<Item = &[Day]> {
        self.chunk_by(|a, b| a.full_date().month() == b.full_date().month())
    }
}

enum Shift {
    Night,
    Day,
}

pub(crate) struct DaysBuilder {
    holidays: HashSet<NaiveDate>,
    first_january_shift: Option<Shift>,
}

impl DaysBuilder {
    pub(crate) fn new() -> Self {
        Self {
            holidays: HashSet::new(),
            first_january_shift: None,
        }
    }

    pub(crate) fn with_holidays(mut self, holidays: &HashSet<NaiveDate>) -> Self {
        self.holidays = holidays.to_owned();
        self
    }

    pub(crate) fn with_shift_type(mut self, work_shift: WorkShift) -> Self {
        let (date, current_shift) = match work_shift {
            WorkShift::IsNight(date) => (date, Shift::Night),
            WorkShift::IsDay(date) => (date, Shift::Day),
        };

        let first_january = NaiveDate::from_ymd_opt(date.year(), 1, 1).unwrap();
        let current_week = date.week(first_january.weekday());
        let shifts_gone = current_week.first_day().ordinal() / 7;

        let first_jan_shift = match (shifts_gone % 2, current_shift) {
            (0, shift) => shift,
            (_, Shift::Night) => Shift::Day,
            (_, Shift::Day) => Shift::Night,
        };

        self.first_january_shift = Some(first_jan_shift);
        self
    }

    pub(crate) fn build(self) -> Days {
        let current_year = match self.holidays.iter().next().cloned() {
            Some(v) => v.year(),
            None => Local::now().date_naive().year(),
        };

        let first_date = NaiveDate::from_ymd_opt(current_year as i32, 1, 1).unwrap();
        let first_jan_weekday = first_date.weekday();

        let days: Days = first_date
            .iter_days()
            .take_while(|d| d.year() == current_year as i32)
            .map(|d| {
                let is_night = match &self.first_january_shift {
                    Some(first_jan_shift) => {
                        let current_week = d.week(first_jan_weekday);
                        let shifts_gone = current_week.first_day().ordinal0() / 7;

                        match (shifts_gone % 2, first_jan_shift) {
                            (0, Shift::Night) => true,
                            (0, Shift::Day) => false,
                            (_, Shift::Night) => false,
                            (_, Shift::Day) => true,
                        }
                    }
                    None => false,
                };

                let day_type = if d.weekday() == Weekday::Sun {
                    DayType::Weekend
                } else if d.weekday() == Weekday::Sat && is_night {
                    DayType::Weekend
                } else if d.weekday() == Weekday::Sat || self.holidays.contains(&d) {
                    DayType::Earn
                } else {
                    DayType::Usual
                };

                Day::new(d, day_type, is_night)
            })
            .collect();

        days
    }
}
