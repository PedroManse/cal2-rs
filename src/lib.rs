use chrono::{DateTime, Month, Utc, Weekday};

pub enum MonthlyEventOverflow {
    Ignore,
    OverflowMonth{overflow_year: bool},
    LastDay,
}

pub enum Event {
    Single(DateTime<Utc>),
    Weekly(Weekday),
    Monthly{
        day: u8,
        overflow_handler: MonthlyEventOverflow,
    },
    Yearly{
        month: Month,
        day: u8,
    },
    Code(mlua::Function),
}

