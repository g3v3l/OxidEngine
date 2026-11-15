use std::time::{Duration, Instant};

pub struct Time{
    last_update: Instant,
    delta: Duration,
    accumulated: Duration,
}

impl Time {
    pub fn new() -> Self {
        Self {
            last_update: Instant::now(),
            delta: Duration::ZERO,
            accumulated: Duration::ZERO,
        }
    }
    pub fn update(&mut self) {
        let now = Instant::now();

        self.delta = now.duration_since(self.last_update);
        self.last_update = now;

        self.accumulated += self.delta;
    }
    pub fn delta(&self) -> Duration {
        self.delta
    }
    pub fn accumulated(&self) -> Duration {
        self.accumulated
    }
    pub fn consume(&mut self, duration: Duration) -> bool {
        if self.accumulated >= duration {
            self.accumulated -= duration;
            true
        } else {
            false
        }
    }
    pub fn clear(&mut self) {
        self.accumulated = Duration::ZERO;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_update() {
        let mut time = Time::new();
        std::thread::sleep(std::time::Duration::from_millis(100));
        time.update();
        assert!(time.delta() > std::time::Duration::ZERO);
    }

    #[test]
    fn test_time_accumulated() {
        let mut time = Time::new();
        std::thread::sleep(std::time::Duration::from_millis(100));
        time.update();
        assert!(time.accumulated() > std::time::Duration::ZERO);
    }

    #[test]
    fn test_time_consume() {
        let mut time = Time::new();
        std::thread::sleep(std::time::Duration::from_millis(100));
        time.update();

        let accumulated_before = time.accumulated();
        let consumed = std::time::Duration::from_millis(50);
        assert!(time.consume(consumed));
        assert_eq!(time.accumulated(), accumulated_before - consumed);
    }

    #[test]
    fn test_time_clear() {
        let mut time = Time::new();
        std::thread::sleep(std::time::Duration::from_millis(100));
        time.update();
        time.clear();
        assert_eq!(time.accumulated(), std::time::Duration::ZERO);
    }
    #[test]
    fn test_time_consume_insufficient() {
        let mut time = Time::new();
        std::thread::sleep(std::time::Duration::from_millis(100));
        time.update();
        let duration = time.accumulated() + std::time::Duration::from_millis(1);
        let accumulated_before = time.accumulated();
        assert!(!time.consume(duration));
        assert_eq!(time.accumulated(), accumulated_before);
    }
}