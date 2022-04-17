use std::fmt::{Display, Formatter, Result};

const MINUTES_IN_DAY: i32 = 1440;
const MINUTES_IN_HOUR: i32 = 60;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = normalize_minutes(hours * MINUTES_IN_HOUR + minutes);

        Clock {
            minutes: total_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(
            formatter,
            "{:02}:{:02}",
            self.minutes / MINUTES_IN_HOUR,
            self.minutes % MINUTES_IN_HOUR
        )
    }
}

impl From<Clock> for String {
    fn from(clock: Clock) -> Self {
        clock.to_string()
    }
}

fn normalize_minutes(mut minutes: i32) -> i32 {
    if minutes < 0 {
        minutes = MINUTES_IN_DAY + (minutes % MINUTES_IN_DAY);
    }

    minutes % MINUTES_IN_DAY
}
