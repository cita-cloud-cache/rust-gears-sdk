use std::time::Instant;

pub struct StatTime {
    pub time_begin: Instant,
    pub time_end: Instant,
}

impl StatTime {
    pub fn begin() -> Self {
        StatTime {
            time_begin: Instant::now(),
            time_end: Instant::now(),
        }
    }
    pub fn done(&mut self) {
        self.time_end = Instant::now();
    }
    pub fn used_ms(&self) -> i64 {
        let time_used = self.time_end - self.time_begin;
        return time_used.as_millis() as i64;
    }
}
