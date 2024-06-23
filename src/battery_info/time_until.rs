use std::{ops::Deref, time::Duration};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeUntil {
    Full(Duration),
    Empty(Duration),
    Unknown(Duration),
}

impl TimeUntil {
    pub fn as_str(&self) -> &str {
        match self {
            TimeUntil::Full(_) => "TimeToFull",
            TimeUntil::Empty(_) => "TimeToEmpty",
            TimeUntil::Unknown(_) => "Unknown",
        }
    }
}

impl Deref for TimeUntil {
    type Target = Duration;

    fn deref(&self) -> &Self::Target {
        match self {
            TimeUntil::Full(d) => d,
            TimeUntil::Empty(d) => d,
            TimeUntil::Unknown(d) => d,
        }
    }
}

impl From<(&str, Duration)> for TimeUntil {
    fn from(value: (&str, Duration)) -> Self {
        let (key, duration) = value;

        match key {
            "TimeToFull" => Self::Full(duration),
            "TimeToEmpty" => Self::Empty(duration),
            &_ => Self::Unknown(duration),
        }
    }
}
