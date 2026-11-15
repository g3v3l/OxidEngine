use std::time::Duration;

pub struct Tick{
    duration: Duration,
}

impl Tick {
    pub fn new(duration: Duration) -> Result<Self, String> {
        if duration <= Duration::ZERO {
            return Err("Duration must be greater than zero".into());
        }
        Ok(Self { duration })
    }

    pub fn duration(&self) -> Duration {
        self.duration
    }
    pub fn from_hz(hz: f64) -> Result<Self, String> {
        if hz <= 0.0 {
            return Err("Frequency must be greater than zero".into());
        }
        if !hz.is_finite() {
            return Err("Frequency must be a finite number".into());
        }
        let seconds_per_tick = 1.0 / hz;
        if seconds_per_tick <= 0.0 {
            return Err("Frequency is too high, resulting in a non-positive duration".into());
        }
        if seconds_per_tick.is_infinite() {
            return Err("Frequency is too low, resulting in an infinite duration".into());
        }
        let duration = Duration::try_from_secs_f64(seconds_per_tick).map_err(|_| "Failed to convert frequency to duration".to_string())?;
        Self::new(duration)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tick_duration() {
        let tick = Tick::new(Duration::from_millis(100)).unwrap();
        assert_eq!(tick.duration(), Duration::from_millis(100));
    }

    #[test]
    fn test_tick_from_hz() {
        let tick = Tick::from_hz(10.0).unwrap();
        assert_eq!(tick.duration(), Duration::from_millis(100));
    }

    #[test]
    fn test_tick_from_hz_invalid() {
        assert!(Tick::from_hz(0.0).is_err());
        assert!(Tick::from_hz(-1.0).is_err());
        assert!(Tick::from_hz(f64::INFINITY).is_err());
        assert!(Tick::new(Duration::ZERO).is_err());
    }
}