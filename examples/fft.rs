use grounded::{complex::Complex, fft::fft, oscillator::Oscillator};

fn main() {
    let mut osc = Oscillator::new(48000.0, 1200.0);

    let mut buffer = [Complex::ZERO; 128];

    for slot in &mut buffer {
        *slot = Complex::new(osc.next().unwrap(), 0.0);
    }

    fft(&mut buffer);

    for sample in &buffer {
        println!("{},{}", sample.re, sample.im);
    }
}
