use num::{complex::Complex, integer::sqrt, Zero};
use std::f64::consts::{E, PI};

/// basic discrete fourier transform
pub fn dft(x: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let l = x.len();

    let mut dft = vec![Complex { re: 0.0, im: 0.0 }; l];
    let c = Complex { re: 0.0, im: -2.0 };
    for k in 0..l {
        let mut t = Complex { re: 0.0, im: 0.0 };
        for n in 0..l {
            t += x[n] * (c * PI * (k as f64) * (n as f64) / (l as f64)).exp();
        }
        dft[k] = t;
    }
    return dft;
}

/// fast fourier transform. Outputs a list that is the length of
/// the next nearest power of 2. The output is suitable for input into
/// the ifft algorithm.
pub fn fft(x: &Vec<f64>) -> Vec<Complex<f64>> {
    // convert to complex
    let mut input = x
        .iter()
        .map(|v| Complex::new(*v, 0.))
        .collect::<Vec<Complex<f64>>>();

    // pad unitl next power of 2
    input.resize(x.len().next_power_of_two(), Complex::zero());

    let t = fft_helper(&input);
    return t;
}
fn fft_helper(x: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let l = x.len();

    if l == 1 {
        return x.to_owned();
    }

    // divide into even and odd parts
    // and run fft on them
    let even: Vec<Complex<f64>> = x.iter().step_by(2).cloned().collect();
    let odd: Vec<Complex<f64>> = x.iter().skip(1).step_by(2).cloned().collect();
    let even = fft_helper(&even);
    let odd = fft_helper(&odd);

    // compute the dft
    let mut dft = vec![Complex { im: 0.0, re: 0.0 }; l];
    let c = Complex { im: -2.0, re: 0.0 };
    for k in 0..(l / 2) {
        let t = odd[k] * (c * PI * (k as f64) / (l as f64)).exp();
        dft[k] = even[k] + t;
        dft[k + l / 2] = even[k] - t;
    }

    return dft;
}

/// perform the inverse fourier transform. This expects that the list
/// is a power of two in length, the fft function assures that.
pub fn ifft(x: &Vec<Complex<f64>>) -> Vec<f64> {
    if !x.len().is_power_of_two() {
        panic!("Input must have a length that is a power of 2");
    } else {
        let out = ifft_helper(x);
        return out
            .to_owned()
            .into_iter()
            .map(|v| v.norm())
            .collect::<Vec<f64>>();
    }
}
fn ifft_helper(x: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let l = x.len();

    if l == 1 {
        return x.to_owned();
    }

    // divide into even and odd parts
    // and run ifft on them
    let even: Vec<Complex<f64>> = x.iter().step_by(2).cloned().collect();
    let odd: Vec<Complex<f64>> = x.iter().skip(1).step_by(2).cloned().collect();
    let even = ifft_helper(&even);
    let odd = ifft_helper(&odd);

    // compute the idft
    let mut idft = vec![Complex { im: 0.0, re: 0.0 }; l];
    let c = Complex { im: 2.0, re: 0.0 };
    for k in 0..(l / 2) {
        let t = odd[k] * (c * PI * (k as f64) / (l as f64)).exp();
        idft[k] = (even[k] + t) / 2.0;
        idft[k + l / 2] = (even[k] - t) / 2.0;
    }

    return idft;
}
