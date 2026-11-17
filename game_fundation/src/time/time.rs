use std::time::{Duration, Instant};
use crate::time::fixedstep::FixedStep;

const MAX_DELTA: Duration = Duration::from_millis(250); // Limitar el delta máximo a 250 ms para evitar saltos grandes

pub struct FixedTime {
    step: FixedStep,
    accumulated: Duration,
    max_accumulated: Option<Duration>,
}

impl FixedTime {
    pub fn new(step: FixedStep) -> Self {
        Self {
            step,
            accumulated: Duration::ZERO,
            max_accumulated: Some(step.duration().saturating_mul(5)), // Limitar la acumulación a 5 pasos
        }
    }

    pub fn accumulate(&mut self, delta: Duration) {
        self.accumulated = self.accumulated.saturating_add(delta).min(self.max_accumulated.unwrap_or(Duration::MAX));
    }

    pub fn expend(&mut self) -> bool {
        match self.accumulated.checked_sub(self.step.duration()) {
            Some(rest) => {
                self.accumulated = rest;
                true
            }
            None => false,
        }
    }

    pub fn step(&self) -> FixedStep {
        self.step
    }

    pub fn overstep_fraction(&self) -> f32 {
        self.accumulated.as_secs_f32() / self.step.duration().as_secs_f32()
    }

    pub fn set_max_accumulated(&mut self, max: Duration) {
        if max < self.step.duration() {
            panic!("max_accumulated must be greater than or equal to step duration");
        }
        self.max_accumulated = Some(max);
    }
}

pub struct Time {
    scale: f32,
    new_scale: Option<f32>,
    delta: Duration,
    unscaled_delta: Duration,
    last_time: Instant,
}

impl Time {
    pub fn new() -> Self {
        Self {
            scale: 1.0,
            new_scale: None,
            delta: Duration::ZERO,
            unscaled_delta: Duration::ZERO,
            last_time: Instant::now(),
        }
    }

    pub fn get_unscaled_delta(&self) -> Duration {
        self.unscaled_delta
    }
    pub fn get_delta(&self) -> Duration {
        self.delta
    }
    pub fn get_scale(&self) -> f32 {
        self.scale
    }
    pub fn set_scale(&mut self, scale: f32) -> Result<(), String> {
    
        if !scale.is_finite() || scale < 0.0 {
            return Err("Scale must be a positive finite number".into());
        }

        if scale >= 10.0 {
            return Err("Scale is too high, it may cause instability".into());
        }

        self.new_scale = Some(scale);
        Ok(())
    }
    fn set_delta(&mut self, delta: Duration) {
        let delta = delta.min(MAX_DELTA);
        self.delta = delta.mul_f32(self.scale);
        self.unscaled_delta = delta;
    }
    pub fn update_time(&mut self, current_time: Instant) {
        let unscaled_delta = current_time.saturating_duration_since(self.last_time);
        self.last_time = current_time;
        if let Some(new_scale) = self.new_scale.take() {
            self.scale = new_scale;
        }
        self.set_delta(unscaled_delta);
    }
    pub fn get_current_time(&self) -> Instant {
        self.last_time
    }
}

pub struct Clock{
    elapsed: Duration,
    step: FixedStep,
}

impl Clock {
    pub fn new(step: FixedStep) -> Self {
        Self {
            elapsed: Duration::ZERO,
            step,
        }
    }
    pub fn advance_time(&mut self, time: &mut Time) {
        let current_time = time.get_current_time() + self.step.duration();
        time.update_time(current_time);
        self.elapsed += self.step.duration();
    }
    pub fn set_step(&mut self, step: FixedStep) {
        self.step = step;
    }
}

pub struct Coldown {
    target: Duration,
    clock: Clock,
}

impl Coldown {
    pub fn new(target: Duration, step: FixedStep) -> Self {
        Self {
            target,
            clock: Clock::new(step),
        }
    }

    pub fn update(&mut self, time: &mut Time) -> bool {
        self.clock.advance_time(time);
        self.clock.elapsed >= self.target
    }

    pub fn reset(&mut self) {
        self.clock.elapsed = Duration::ZERO;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ms(n: u64) -> Duration { Duration::from_millis(n) }

    #[test]
    fn zero_scale_pauses() {
        let base = Instant::now();
        let mut t = Time::new();
        t.update_time(base);
        t.set_scale(0.0).unwrap();
        t.update_time(base + ms(16));
        assert_eq!(t.get_delta(), Duration::ZERO);
        assert_eq!(t.get_unscaled_delta(), ms(16));
    }

    #[test]
    fn scale_applies_next_frame() {
        let base = Instant::now();
        let mut t = Time::new();
        t.update_time(base);
        t.set_scale(2.0).unwrap();
        assert_eq!(t.get_scale(), 1.0);
        t.update_time(base + ms(10));
        assert_eq!(t.get_scale(), 2.0);
        assert_eq!(t.get_delta(), ms(20));
    }

    #[test]
    fn hitch_is_clamped() {
        let base = Instant::now();
        let mut t = Time::new();
        t.update_time(base);
        t.update_time(base + Duration::from_secs(2));
        assert_eq!(t.get_unscaled_delta(), ms(250));
    }

    #[test]
    fn fixed_clamps_accumulation() {
        let mut f = FixedTime::new(FixedStep::from_hz(60.0).unwrap());
        f.accumulate(Duration::from_secs(10));
        let mut n = 0;
        while f.expend() { n += 1; }
        assert_eq!(n, 5);
    }

    #[test]
    fn fixed_expends_exact_steps() {
        let mut f = FixedTime::new(FixedStep::from_hz(100.0).unwrap()); // 10 ms
        f.accumulate(ms(35));
        let mut n = 0;
        while f.expend() { n += 1; }
        assert_eq!(n, 3);
        let frac = f.overstep_fraction();
        assert!(frac > 0.4 && frac < 0.6);
    }
}