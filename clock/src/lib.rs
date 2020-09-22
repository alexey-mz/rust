use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let temp_hours = ((hours * 60  + minutes) % 1440 + 1440) / 60 % 24;
        let temp_minutes = ((hours * 60 + minutes) % 1440 + 1440) % 60;
        Self {
            hours: temp_hours,
            minutes: temp_minutes
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours * 60 + self.minutes == other.hours * 60 + other.minutes
    }
}