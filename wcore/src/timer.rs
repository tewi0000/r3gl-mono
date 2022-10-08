use instant::{Instant, Duration};

pub struct Timer {
    last_pause: Instant,
    last_time: Duration,
}

impl Timer {
    pub fn new() -> Self {
        return Self {
            last_pause: Instant::now(),
            last_time: Duration::ZERO,
        };
    }

    pub fn sync(&self, paused: bool, time: Duration) -> Duration {
        if paused { return time; } else {
            let now = instant::Instant::now();
            let diff = now.duration_since(self.last_pause);
            return diff + self.last_time;
        };
    }

    pub fn reset(&mut self, time: Duration) {
        self.last_pause = Instant::now();
        self.last_time = time;
    }
}