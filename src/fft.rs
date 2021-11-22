use std::f64::consts::TAU;

use crate::complex::Complex;

pub fn fft(nums: &mut [Complex]) {
    let n = nums.len();
    assert!(n.is_power_of_two());
    let bits = usize::BITS - (n.leading_zeros() + 1);

    let bit_reverse = |i: usize| i.reverse_bits() >> (usize::BITS - bits);
    for i in 0..nums.len() {
        let j = bit_reverse(i);
        // Otherwise, pairs would be visited (and swapped) twice:
        if j > i {
            nums.swap(i, j);
        }
    }

    for s in 1..=bits {
        let m = 2usize.pow(s);
        let om = Complex::polar(1.0, -TAU / (m as f64));
        for k in (0..n).step_by(m) {
            let mut o = Complex::ONE;
            for j in 0..(m / 2) {
                let first = k + j;
                let second = first + m / 2;

                let t = nums[first];
                let u = o * nums[second];
                nums[first] = t + u;
                nums[second] = t - u;
                o *= om;
            }
        }
    }
}
