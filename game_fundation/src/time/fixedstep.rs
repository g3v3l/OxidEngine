use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FixedStep {
    duration: Duration,
}

impl FixedStep {
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
        if !hz.is_finite() {
            return Err("Frequency must be a finite number".into());
        }
        if hz <= 0.0 {
            return Err("Frequency must be greater than zero".into());
        }
        let seconds_per_tick = 1.0 / hz;
        let duration = Duration::try_from_secs_f64(seconds_per_tick).map_err(|_| "Failed to convert frequency to duration".to_string())?;
        if duration <= Duration::ZERO {
            return Err("Calculated duration must be greater than zero".into());
        }
        Self::new(duration)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_step_duration() {
        let step = FixedStep::new(Duration::from_millis(100)).unwrap();
        assert_eq!(step.duration(), Duration::from_millis(100));
    }

    #[test]
    fn test_fixed_step_from_hz() {
        let step = FixedStep::from_hz(10.0).unwrap();
        assert_eq!(step.duration(), Duration::from_millis(100));
    }

    #[test]
    fn test_fixed_step_from_hz_invalid() {
        assert!(FixedStep::from_hz(0.0).is_err());
        assert!(FixedStep::from_hz(-1.0).is_err());
        assert!(FixedStep::from_hz(f64::INFINITY).is_err());
        assert!(FixedStep::new(Duration::ZERO).is_err());
    }
}