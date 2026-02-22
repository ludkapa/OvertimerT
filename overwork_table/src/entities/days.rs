use chrono::{Datelike, Local, NaiveDate, Weekday};
use derive_more::{Deref, DerefMut, IntoIterator};
use std::collections::HashSet;

#[derive(Default, Debug, Clone, Copy)]
pub(crate) enum DayType {
    #[default]
    Usual,
    Earn,
    Weekend,
}

pub(crate) enum Season {
    Winter,
    Spring,
    Summer,
    Autumn,
}

#[derive(Default, Debug)]
pub(crate) struct Day {
    day: NaiveDate,
    flag: DayType,
}

impl Day {
    pub(crate) fn new(day: NaiveDate, flag: DayType) -> Self {
        Self { day, flag }
    }

    pub(crate) fn year(&self) -> i32 {
        self.day.year()
    }

    pub(crate) fn earn_type(&self) -> DayType {
        self.flag
    }

    pub(crate) fn number(&self) -> u32 {
        self.day.day()
    }

    pub(crate) fn weekday_short(&self) -> String {
        match self.day.weekday() {
            Weekday::Mon => "Пн".to_string(),
            Weekday::Tue => "Вт".to_string(),
            Weekday::Wed => "Ср".to_string(),
            Weekday::Thu => "Чт".to_string(),
            Weekday::Fri => "Пт".to_string(),
            Weekday::Sat => "Сб".to_string(),
            Weekday::Sun => "Вс".to_string(),
        }
    }

    pub(crate) fn month_name(&self) -> String {
        match self.day.month() {
            1 => "❄️ Январь".to_string(),
            2 => "🌨️ Февраль".to_string(),
            3 => "🌱 Март".to_string(),
            4 => "🌸 Апрель".to_string(),
            5 => "🌿 Май".to_string(),
            6 => "☀️ Июнь".to_string(),
            7 => "🏖️ Июль".to_string(),
            8 => "🍉 Август".to_string(),
            9 => "🍂 Сентябрь".to_string(),
            10 => "🍁 Октябрь".to_string(),
            11 => "🌧️ Ноябрь".to_string(),
            12 => "🎄 Декабрь".to_string(),
            _ => "❓ Неизвестный месяц".to_string(),
        }
    }

    pub(crate) fn season(&self) -> Season {
        match self.day.month() {
            1 | 2 | 12 => Season::Winter,
            3 | 4 | 5 => Season::Spring,
            6 | 7 | 8 => Season::Summer,
            9 | 10 | 11 => Season::Autumn,
            _ => Season::Winter,
        }
    }
}

#[derive(IntoIterator, Deref, DerefMut)]
pub(crate) struct Days(Vec<Day>);

impl FromIterator<Day> for Days {
    fn from_iter<T: IntoIterator<Item = Day>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Days {
    pub(crate) fn from_holidays(holidays: &HashSet<NaiveDate>) -> Self {
        let current_year = match holidays.iter().next().cloned() {
            Some(v) => v.year(),
            None => Local::now().date_naive().year(),
        };
        let first_date = NaiveDate::from_ymd_opt(current_year as i32, 1, 1).unwrap();
        let days: Days = first_date
            .iter_days()
            .take_while(|d| d.year() == current_year as i32)
            .map(|d| {
                if d.weekday() == Weekday::Sun {
                    Day::new(d, DayType::Weekend)
                } else if d.weekday() == Weekday::Sat || holidays.contains(&d) {
                    Day::new(d, DayType::Earn)
                } else {
                    Day::new(d, DayType::Usual)
                }
            })
            .collect();
        days
    }

    pub(crate) fn split_months(&self) -> impl Iterator<Item = &[Day]> {
        self.chunk_by(|a, b| a.day.month() == b.day.month())
    }
}
