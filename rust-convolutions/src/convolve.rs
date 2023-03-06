use crate::dft;
use crate::padding;
use num::complex::Complex;
use num::Zero;
use rustfft::FftPlanner;

/// perform a basic convolution with no padding
pub fn conv(input: &Vec<f64>, kernel: &Vec<f64>) -> Vec<f64> {
    let mut out: Vec<f64> = vec![];

    for i in 0..input.len() - kernel.len() {
        // internal dot product
        let mut val = 0.;
        for j in 0..kernel.len() {
            val += input[i + j] * kernel[j];
        }
        out.push(val);
    }
    return out;
}

pub fn conv_pad(input: &Vec<f64>, kernel: &Vec<f64>) -> Vec<f64> {
    // pad the input with zeros on both sides
    let mut padded_input: Vec<f64> = vec![0.; input.len() + kernel.len()];
    for i in 0..input.len() {
        padded_input[(kernel.len() / 2) + i] = input[i];
    }

    // perform a normal convolution on the padded input
    let mut out: Vec<f64> = vec![];

    for i in 0..padded_input.len() - kernel.len() {
        let mut val = 0.;
        for j in 0..kernel.len() {
            val += padded_input[i + j] * kernel[j];
        }
        out.push(val);
    }
    return out;
}

/// Use basic matrix multiplication to calculate the step-wise convolution of
/// an input vector and a kernel. The output will be the same size as the
/// param `input`
pub fn conv_2d(input: &Vec<Vec<f64>>, kernel: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    // create zero padded version of input list to account for
    // kernel size
    let mut padded = padding::reflection_pad(input, kernel.len());

    let mut out = vec![vec![0.0; input[0].len()]; input.len()];

    // loop over the range of pixels calculate the matrix
    // product using the kernel
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let mut sum = 0.0;
            // calculate sum of sub-list and kernel
            for n in 0..kernel.len() {
                for m in 0..kernel[0].len() {
                    sum += padded[i + n][j + m] * kernel[n][m];
                }
            }
            out[i][j] = sum;
        }
    }

    return out;
}

/// Uses the fast fourier transform algorithm to calculate the
/// convolution between an image `input` and a `kernel`
pub fn fft_conv_2d(input: &Vec<Vec<f64>>, kernel: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    // get image dimmensions
    let width = input[0].len();
    let height = input.len();

    // flatten image
    let mut image = input.to_owned().into_iter().flatten().collect::<Vec<f64>>();

    // flatten kernel
    let mut flat_kernel = kernel
        .to_owned()
        .into_iter()
        .flatten()
        .collect::<Vec<f64>>();

    let mut padded_kernel = vec![0.; image.len()];
    for i in 0..flat_kernel.len() {
        padded_kernel[i] = flat_kernel[i];
    }

    // perform ffts
    let image_fft = dft::fft(&image);
    let kernel_fft = dft::fft(&padded_kernel);

    // multiply the fft together
    let result_fft: Vec<Complex<f64>> = image_fft
        .iter()
        .zip(kernel_fft.iter())
        .map(|(a, b)| a * b)
        .collect();

    // perform ifft
    let result_ifft = dft::ifft(&result_fft);

    // reconstruct the 2d vector
    let mut result_norm = vec![vec![0.; width]; height];
    for y in 0..height {
        for x in 0..width {
            result_norm[y][x] = result_ifft[y as usize * width + x as usize]
        }
    }

    return result_norm;
}

/// uses faster fft algorithms to calculate the convolution between an image and a
/// kernel.
pub fn fft_conv_2d_fast(input: &Vec<Vec<f64>>, kernel: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    // get image dimmensions
    let width = input[0].len();
    let height = input.len();

    // flatten image
    let mut image = input
        .to_owned()
        .into_iter()
        .flatten()
        .map(|v| Complex::new(v, 0.0))
        .collect::<Vec<Complex<f64>>>();

    // flatten kernel
    let flat_kernel = kernel
        .to_owned()
        .into_iter()
        .flatten()
        .collect::<Vec<f64>>();

    // let mut padded_kernel = vec![Complex::zero(); image.len()];
    // for i in 0..flat_kernel.len() {
    //     padded_kernel[i] = Complex::new(flat_kernel[i], 0.0);
    // }

    // pad kernel with reflection padding
    let mut padded_kernel = vec![Complex::zero(); image.len()];
    let kernel_width = kernel[0].len();
    let kernel_height = kernel.len();
    let pad_x = kernel_width / 2;
    let pad_y = kernel_height / 2;
    for y in 0..kernel_height {
        for x in 0..kernel_width {
            let kernel_val = kernel[y][x];
            let padded_index = (y + pad_y) * width + (x + pad_x);
            padded_kernel[padded_index] = Complex::new(kernel_val, 0.0);
            if x < pad_x {
                let mirror_index = (y + pad_y) * width + (pad_x - x - 1);
                padded_kernel[mirror_index] = Complex::new(kernel_val, 0.0);
            }
            if y < pad_y {
                let mirror_index = (pad_y - y - 1) * width + (x + pad_x);
                padded_kernel[mirror_index] = Complex::new(kernel_val, 0.0);
            }
            if x < pad_x && y < pad_y {
                let mirror_index = (pad_y - y - 1) * width + (pad_x - x - 1);
                padded_kernel[mirror_index] = Complex::new(kernel_val, 0.0);
            }
        }
    }

    // perform ffts
    let mut planner = FftPlanner::new();
    let f = planner.plan_fft_forward(image.len());

    f.process(&mut image);
    f.process(&mut padded_kernel);

    // multiply the fft together
    let mut result_fft: Vec<Complex<f64>> = image
        .iter()
        .zip(padded_kernel.iter())
        .map(|(a, b)| a * b)
        .collect();

    let f = planner.plan_fft_inverse(image.len());

    // perform ifft
    f.process(&mut result_fft);

    // reconstruct the 2d vector
    let max_value = result_fft.iter().map(|c| c.norm()).fold(0.0, f64::max);
    let mut result_norm = vec![vec![0.; width]; height];
    for y in 0..height {
        for x in 0..width {
            result_norm[y][x] = result_fft[y as usize * width + x as usize].norm() / max_value
        }
    }

    return result_norm;
}

pub fn print_vec_2d(list: &Vec<Vec<f64>>) {
    for i in 0..list.len() {
        if i == 0 {
            print!("[[");
        } else {
            print!(" [");
        }
        for j in 0..list[0].len() {
            if (list[i][j] < 10.) {
                print!("0");
            }
            print!("{:.1}, ", list[i][j]);
        }
        if (i == list.len() - 1) {
            print!("]] ({:?}x{:?})\n", list.len(), list[0].len());
        } else {
            print!("]\n");
        }
    }
}
