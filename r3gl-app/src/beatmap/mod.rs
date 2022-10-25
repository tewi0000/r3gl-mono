use instant::Duration;

pub mod component;
pub mod parser;
pub mod beatmap;

#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct Time(u32); // in milliseconds

impl From<Duration> for Time {
    fn from(value: Duration) -> Self {
        return Time(value.as_millis() as u32);
    }
}

impl Time {
    pub fn from_ms(value: u32) -> Self {
        return Time(value);
    }

    pub fn as_ms(&self) -> u32 {
        return self.0;
    }
}