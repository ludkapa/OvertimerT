use chrono::{Datelike, NaiveDate, Weekday};

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

    pub(crate) fn full_date(&self) -> NaiveDate {
        self.day
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
