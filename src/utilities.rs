extern crate array2;
use array2::Array2;
use csc411_image::{Rgb, RgbImage};

pub fn crop(src: &RgbImage) -> Array2<Rgb> {
    let mut crop_width = src.width;
    let mut crop_height = src.height;
    // configure height and width to be even
    if src.width % 2 == 1 {
        crop_width = src.width - 1;
    }

    if src.height % 2 == 1 {
        crop_height = src.height - 1;
    }

    let mut dest = Array2::new(crop_width as usize, crop_height as usize, Rgb { red: 0, green: 0, blue: 0 });
    let dest_height = dest.height;
    let dest_width = dest.width;
    for (i, dest_pixel) in dest.iter_row_major().enumerate() {
        let row = i / dest_height;
        let col = i % dest_height;
        *dest_pixel = src.pixels[row * dest_width + col].clone();
    }
    dest
}

pub fn print_ppm_as_rgb_array2(ppm_crop: &Array2<Rgb>) {
    let (rows, cols) = (ppm_crop.rows(), ppm_crop.cols());
    let num_elements = rows * cols;
        println!("Number of Rgb elements in cropped ppm: {}", num_elements);

    // print pixel values
    // UNCERTAIN IF WIDTH AND HEIGHT IS CORRECT
    for row in 0..ppm_crop.width {
        for col in 0..ppm_crop.height {
            let pixel = ppm_crop.get(row,col).unwrap();
            print!("[Red: {}, Green: {}, Blue: {}]", pixel.red, pixel.green, pixel.blue);
        }
        println!();
    }
}

pub fn print_ppm_as_float_array2(ppm_crop: &Array2<Vec<f64>>) {
    let (rows, cols) = (ppm_crop.rows(), ppm_crop.cols());
    let num_elements = rows * cols;
        println!("Number of Rgb elements in cropped ppm: {}", num_elements);

    // print pixel values
    // UNCERTAIN IF WIDTH AND HEIGHT IS CORRECT
    for row in 0..ppm_crop.width {
        for col in 0..ppm_crop.height {
            let pixel = ppm_crop.get(row,col).unwrap();
            print!("[Red: {}, Green: {}, Blue: {}]", pixel[0], pixel[1], pixel[2]);
        }
        println!();
    }
}

pub fn rgb_int_to_float(ppm_crop: &Array2<Rgb>, denom: u16) -> Array2<Vec<f64>> {
    let mut ppm_float = Array2::new(ppm_crop.width as usize, ppm_crop.height as usize, Vec::new());
    //iterate and change pixels r, g, b values to float
    let height = ppm_float.height;
    for (i, dest_pixel) in ppm_float.iter_row_major().enumerate() {
        let row = i / height;
        let col = i % height;
        let pixel = ppm_crop.get(row, col).unwrap();
        // update pixel values as float to Vec<vec<f64>>
        *dest_pixel = vec![pixel.red as f64 / denom as f64, pixel.green as f64 / denom as f64, pixel.blue as f64 / denom as f64];
    }

    ppm_float
}

pub fn rgb_float_to_int(ppm_float: &Array2<Vec<f64>>, denom: u16) -> Array2<Rgb> {
    let mut ppm_int = Array2::new(ppm_float.width as usize, ppm_float.height as usize, Rgb { red: 0, green: 0, blue: 0 });
    //iterate and change pixels r, g, b values to int
    let height = ppm_int.height;
    for (i, dest_pixel) in ppm_int.iter_row_major().enumerate() {
        let row = i / height;
        let col = i % height;
        let pixel = ppm_float.get(row, col).unwrap();
        // update pixel values as i32 to Vec<vec<f64>>
        *dest_pixel = Rgb { red: (pixel[0] * denom as f64) as u16, green: (pixel[1] * denom as f64) as u16, blue: (pixel[2] * denom as f64) as u16 };
    }

    ppm_int
}