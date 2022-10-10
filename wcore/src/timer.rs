use instant::{Instant, Duration};

pub struct Timer {
    last_pause: Instant,
    last_time: Duration,
    pub target: Duration,
}

impl Timer {
    pub fn new() -> Self {
        return Self {
            last_pause: Instant::now(),
            last_time: Duration::ZERO,
            target: Duration::ZERO,
        };
    }

    pub fn sync(&mut self, paused: bool, time: Duration) -> Duration {
        if paused {
            let step = ((self.target.as_millis() as i32 - self.last_time.as_millis() as i32) as f32 / 50.0).abs().ceil() as u64;
            self.last_pause = Instant::now();
            if self.last_time < self.target {
                self.last_time = self.last_time.saturating_add(Duration::from_millis(step));
            } else {
                self.last_time = self.last_time.saturating_sub(Duration::from_millis(step));
            }
            self.target = time; 
            self.last_time = self.last_time; 

            self.target = time; 
            return time;
        } else {
            let now = instant::Instant::now();
            let diff = now.duration_since(self.last_pause);
            return diff + self.last_time;
        };
    }

    pub fn reset(&mut self, time: Duration) {
        self.last_pause = Instant::now();
        self.target = time;
    }

    pub fn interpolate(&mut self, time: Duration) {
    }
}