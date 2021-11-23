use std::f64::consts::TAU;

use crate::complex::Complex;

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
    type Item = Complex;

    fn next(&mut self) -> Option<Self::Item> {
        let sample = Complex::polar(1.0, self.phase);
        self.phase += TAU * self.frequency / self.sample_rate;
        self.phase %= TAU;
        Some(sample)
    }
}
