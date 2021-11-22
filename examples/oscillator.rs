use grounded::oscillator::Oscillator;

fn main() {
    let mut osc = Oscillator::new(48000.0, 1200.0);

    for _ in 0..100 {
        println!("{}", osc.next().unwrap());
    }
}
