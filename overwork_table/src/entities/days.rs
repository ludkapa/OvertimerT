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

#[derive(Debug)]
enum Shift {
    Night,
    Day,
}

impl Shift {
    pub fn next(&self) -> Self {
        match self {
            Shift::Night => Shift::Day,
            Shift::Day => Shift::Night,
        }
    }
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

    pub(crate) fn with_shift_type(mut self, work_shift: &WorkShift) -> Self {
        let (date, current_shift) = match work_shift {
            WorkShift::IsNight(date) => (date, Shift::Night),
            WorkShift::IsDay(date) => (date, Shift::Day),
        };

        let day_of_year = date.ordinal();
        let shifts_gone = day_of_year / 7;

        let first_jan_shift = match (shifts_gone % 2, current_shift) {
            (0, shift) => shift,
            (_, shift) => shift.next(),
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

        let mut is_night = match &self.first_january_shift {
            Some(Shift::Night) => true,
            Some(Shift::Day) => false,
            None => false,
        };

        let days: Days = first_date
            .iter_days()
            .take_while(|d| d.year() == current_year as i32)
            .map(|d| {
                is_night = match &self.first_january_shift {
                    Some(_) => {
                        if d.weekday() == Weekday::Mon {
                            !is_night
                        } else {
                            is_night
                        }
                    }
                    None => false,
                };

                let is_holiday = self.holidays.contains(&d);

                let day_type = match (d.weekday(), is_night, is_holiday) {
                    // Sunday
                    (Weekday::Sun, _, _) => DayType::Weekend,

                    // Saturday
                    (Weekday::Sat, true, _) => DayType::Weekend, // Night
                    (Weekday::Sat, false, _) => DayType::Earn,   // Day

                    // Holidays
                    (_, true, true) => DayType::NightEarn,
                    (_, false, true) => DayType::Earn,

                    // Usual days
                    (_, true, false) => DayType::Night,
                    (_, false, false) => DayType::Usual,
                };

                Day::new(d, day_type)
            })
            .collect();

        days
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_jan_shift() {
        // Given
        let holidays_mock: HashSet<NaiveDate> = HashSet::new();
        let start_date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        let jan1_weekday = start_date.weekday().num_days_from_monday();

        start_date
            .iter_days()
            .filter(|d| d.year() == 2026)
            .for_each(|d| {
                let day_of_year = d.ordinal0();
                let weeks_passed = (day_of_year + jan1_weekday) / 7;

                let current_shift = match weeks_passed % 2 {
                    0 => WorkShift::IsDay(d),
                    _ => WorkShift::IsNight(d),
                };

                // when
                let days = DaysBuilder::new()
                    .with_holidays(&holidays_mock)
                    .with_shift_type(&current_shift)
                    .build();

                let current_earn_type = days.first().unwrap().earn_type();
                let target_earn_type = DayType::Usual;

                // Then
                assert_eq!(
                    current_earn_type, target_earn_type,
                    "Ошибка на дате: {} (прошло недель: {})",
                    d, weeks_passed
                );
            });
    }
}
