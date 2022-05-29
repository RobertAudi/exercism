use time::ext::*;
use time::Time;

use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let time = Time::MIDNIGHT + (hours as i64).hours() + (minutes as i64).minutes();
        let hours = time.hour() as i32;
        let minutes = time.minute() as i32;

        Self {
            hours: hours,
            minutes: minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let time = Time::MIDNIGHT
            + (self.hours as i64).hours()
            + (self.minutes as i64).minutes()
            + (minutes as i64).minutes();

        Self {
            hours: time.hour() as i32,
            minutes: time.minute() as i32,
        }
    }
}
