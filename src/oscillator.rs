use std::f64::consts::TAU;

pub struct Oscillator {
    sample_rate: f64,
    frequency: f64,
    phase: f64,
}

impl Oscillator {
    pub fn new(sample_rate: f64, frequency: f64) -> Self {
        Self {
            sample_rate,
            frequency,
            phase: 0.0,
        }
    }
}

impl Iterator for Oscillator {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.phase.sin();
        self.phase += TAU * self.frequency / self.sample_rate;
        self.phase %= TAU;
        Some(sample)
    }
}
