use grounded::{complex::Complex, fft::fft, oscillator::Oscillator};

fn main() {
    let mut osc = Oscillator::new(48000.0, 1200.0);

    let mut buffer = [Complex::ZERO; 128];

    for sample in &mut buffer {
        *sample = osc.next().unwrap();
    }
    let signal = buffer.clone();

    fft(&mut buffer);

    for (sample_a, sample_b) in signal.iter().zip(&buffer) {
        println!(
            "{},{},{},{}",
            sample_a.re, sample_a.im, sample_b.re, sample_b.im
        );
    }
}
