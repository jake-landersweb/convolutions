#![allow(unused)] // for beginning only

use crate::prelude::*;
use image::{GenericImageView, GrayImage};
use std::time::Instant;
mod kernel;
use kernel::*;

mod convolve;
mod dft;
mod error;
mod prelude;

fn main() -> Result<()> {
    let now = Instant::now();
    let img_name = "3840x2160.jpg";
    let kernel = Kernel::gaussian(31, 5.);
    let kernel_name = "gaussian";

    // read the image
    let (width, height, img) = read_image(img_name)?;

    // process image
    let processed_fft = convolve::fft_conv_2d_fast(&img, &kernel);

    // save the images
    let items = img_name.split(".").collect::<Vec<&str>>();
    let img_name = std::format!("{}-{}-fft-fast.{}", items[0], kernel_name, items[1]);
    save_img(width, height, &img_name, &processed_fft)?;

    let elapsed = now.elapsed();
    println!("Basic Convolve: {:.3?} sec", elapsed.as_secs_f32());

    return Ok(());
}

fn main2() -> Result<()> {
    let now = Instant::now();
    let img_name = "3840x2160.jpg";
    let kernel = Kernel::gaussian(31, 5.);
    let kernel_name = "gaussian";

    // read the image
    let (width, height, img) = read_image(img_name)?;

    // process image
    let processed_fft = convolve::conv_2d(&img, &kernel);

    // save the images
    let items = img_name.split(".").collect::<Vec<&str>>();
    let img_name = std::format!("{}-{}.{}", items[0], kernel_name, items[1]);
    save_img(width, height, &img_name, &processed_fft)?;

    let elapsed = now.elapsed();
    println!("Basic Convolve: {:.3?} sec", elapsed.as_secs_f32());

    return Ok(());
}

fn conv_example() {
    let input = vec![0., 0., 0., 4., 4., 4., 4., 0., 0., 4., 4., 4., 0., 0., 0.];
    let kernel: Vec<f64> = vec![1. / 3., 1. / 3., 1. / 3.];
    let convout = convolve::conv(&input, &kernel);
    println!("{:?}", convout);
    println!(
        "Original length: {}, Output Length: {}",
        input.len(),
        convout.len()
    );
}

fn conv_pad_example() {
    let input = vec![0., 0., 0., 4., 4., 4., 4., 0., 0., 4., 4., 4., 0., 0., 0.];
    let kernel: Vec<f64> = vec![1. / 3., 1. / 3., 1. / 3.];
    let convout = convolve::conv_pad(&input, &kernel);
    println!("{:?}", convout);
    println!(
        "Original length: {}, Output Length: {}",
        input.len(),
        convout.len()
    );
}

fn process_example() {
    let imgs = [
        // "256x256.jpg",
        // "1600x900.jpg",
        // "1920x1080.jpg",
        "3840x2160.jpg",
    ];
    let kernel = Kernel::gaussian(31, 5.);
    let kernel_name = "gaussian";
    for img in imgs {
        println!("[[Testing img: {} with kernel: {}]]", img, kernel_name);
        process_image(img, &kernel, kernel_name);
        process_image_rustfft(img, &kernel, kernel_name);
        process_image_cfft(img, &kernel, kernel_name);
        println!();
    }
}

fn process_image(image_name: &str, kernel: &Kernel, kernel_name: &str) -> Result<()> {
    let now = Instant::now();

    // read the image
    let (width, height, img) = read_image(image_name)?;

    // process image
    let processed_fft = convolve::conv_2d(&img, kernel);

    // save the images
    let items = image_name.split(".").collect::<Vec<&str>>();
    let img_name = std::format!("{}-{}.{}", items[0], kernel_name, items[1]);
    save_img(width, height, &img_name, &processed_fft)?;

    let elapsed = now.elapsed();
    println!("Basic Convolve: {:.3?} sec", elapsed.as_secs_f32());

    return Ok(());
}

fn process_image_cfft(image_name: &str, kernel: &Kernel, kernel_name: &str) -> Result<()> {
    let now = Instant::now();

    // read the image
    let (width, height, img) = read_image(image_name)?;

    // process image
    let processed_fft = convolve::fft_conv_2d(&img, kernel);

    // save the images
    let items = image_name.split(".").collect::<Vec<&str>>();
    let img_name = std::format!("{}-{}-cfft.{}", items[0], kernel_name, items[1]);
    save_img(width, height, &img_name, &processed_fft)?;

    let elapsed = now.elapsed();
    println!("Custom FFT: {:.3?} sec", elapsed.as_secs_f32());

    return Ok(());
}

fn process_image_rustfft(image_name: &str, kernel: &Kernel, kernel_name: &str) -> Result<()> {
    let now = Instant::now();

    // read the image
    let (width, height, img) = read_image(image_name)?;

    // process image
    let processed_fft = convolve::fft_conv_2d_fast(&img, kernel);

    // save the images
    let items = image_name.split(".").collect::<Vec<&str>>();
    let img_name = std::format!("{}-{}-rustfft.{}", items[0], kernel_name, items[1]);
    save_img(width, height, &img_name, &processed_fft)?;

    let elapsed = now.elapsed();
    println!("RustFFT: {:.3?} sec", elapsed.as_secs_f32());

    return Ok(());
}

fn read_image(image_name: &str) -> Result<((u32, u32, Vec<Vec<f64>>))> {
    // read the image
    let img = image::open(std::format!(
        "/Users/jakelanders/code/convolutions/rust-convolutions/input_images/{}",
        image_name
    ))?;

    let (width, height) = img.dimensions();

    // create an array to hold the image
    let mut gray_img = vec![vec![0.; width as usize]; height as usize];

    // loop through and calculate the pixel values as 0-1 grey float values
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let rgb = pixel.0;
            let gray =
                (0.2989 * rgb[0] as f64 + 0.5870 * rgb[1] as f64 + 0.1140 * rgb[2] as f64) / 255.;
            gray_img[y as usize][x as usize] = gray;
        }
    }

    return Ok((width, height, gray_img));
}

fn save_img(width: u32, height: u32, image_name: &str, img: &Vec<Vec<f64>>) -> Result<()> {
    // convert to grey image
    let mut ouput = GrayImage::new(width, height);
    for (x, y, pixel) in ouput.enumerate_pixels_mut() {
        let gray_value = (img[y as usize][x as usize] * 255.0) as u8;
        *pixel = image::Luma([gray_value]);
    }

    // save the image
    ouput.save(std::format!(
        "/Users/jakelanders/code/convolutions/rust-convolutions/output_images/{}",
        image_name
    ))?;

    return Ok(());
}
