extern crate array2;
use array2::Array2;
use csc411_image::{Rgb, RgbImage};

/// It takes an image, crops it to be even in width and height, and returns the cropped image
/// 
/// Arguments:
/// 
/// * `src`: The source image to crop
/// 
/// Returns:
/// 
/// A 2D array of RGB values
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
    // print width height of src
    println!("Width: {}, Height: {}", src.width, src.height);
    // print width height of dest
    println!("DWidth: {}, DHeight: {}", dest.width, dest.height);
    let dest_height = dest.height;
    let dest_width = dest.width;
    println!("Length of data: {}", dest.data.len());
    for (i, dest_pixel) in dest.iter_row_major_mut().enumerate() {
        let row = i / dest_width;
        let col = i % dest_width;
        println!("Index: {}, Row: {}, Col: {}", row*dest_width+col, row, col);
        *dest_pixel = src.pixels[row * dest_width + col].clone();
    }
    dest
}

/// `print_ppm_as_rgb_array2` prints the contents of a 2D array of `Rgb` pixels
/// 
/// Arguments:
/// 
/// * `ppm_crop`: &Array2<Rgb>
pub fn print_ppm_as_rgb_array2(ppm_crop: &Array2<Rgb>) {
    let (rows, cols) = (ppm_crop.rows(), ppm_crop.cols());
    let num_elements = rows * cols;
        println!("Number of Rgb elements in cropped ppm: {}", num_elements);

    // print pixel values
    for row in 0..ppm_crop.height {
        for col in 0..ppm_crop.width {
            let pixel = ppm_crop.get(row,col).unwrap();
            print!("[Red: {}, Green: {}, Blue: {}]", pixel.red, pixel.green, pixel.blue);
        }
        println!();
    }
}

/// `print_ppm_as_float_array2` prints the contents of a 2D array of RGB values as a series of 3-tuples
/// 
/// Arguments:
/// 
/// * `ppm_crop`: &Array2<Vec<f64>>
pub fn print_ppm_as_float_array2(ppm_crop: &Array2<Vec<f64>>) {
    let (rows, cols) = (ppm_crop.rows(), ppm_crop.cols());
    let num_elements = rows * cols;
        println!("Number of Rgb elements in cropped ppm: {}", num_elements);

    // print pixel values
    for row in 0..ppm_crop.height {
        for col in 0..ppm_crop.width {
            let pixel = ppm_crop.get(row,col).unwrap();
            print!("[Red: {}, Green: {}, Blue: {}]", pixel[0], pixel[1], pixel[2]);
        }
        println!();
    }
}

/// It takes a 2D array of RGB values and converts them to a 2D array of floating point values
/// 
/// Arguments:
/// 
/// * `ppm_crop`: The cropped image
/// * `denom`: the denominator used to convert the pixel values to float.
/// 
/// Returns:
/// 
/// A 2D array of Vec<f64>
pub fn rgb_int_to_float(ppm_crop: &Array2<Rgb>, denom: u16) -> Array2<Vec<f64>> {
    let mut ppm_float = Array2::new(ppm_crop.width as usize, ppm_crop.height as usize, Vec::new());
    //iterate and change pixels r, g, b values to float
    let width = ppm_float.width;
    for (i, dest_pixel) in ppm_float.iter_row_major_mut().enumerate() {
        let row = i / width;
        let col = i % width;
        let pixel = ppm_crop.get(row, col).unwrap();
        // update pixel values as float to Vec<vec<f64>>
        *dest_pixel = vec![pixel.red as f64 / denom as f64, pixel.green as f64 / denom as f64, pixel.blue as f64 / denom as f64];
    }

    ppm_float
}

/// It takes a 2D array of RGB values as floats, and returns a 2D array of RGB values as integers
/// 
/// Arguments:
/// 
/// * `ppm_float`: The image as a 2D array of RGB values in floating point format.
/// * `denom`: the denominator to multiply with the float value. For example, if the float value is 0.5, the
/// product is the original integer value.
/// 
/// Returns:
/// 
/// A 2D array of RGB values
pub fn rgb_float_to_int(ppm_float: &Array2<Vec<f64>>, denom: u16) -> Array2<Rgb> {
    let mut ppm_int = Array2::new(ppm_float.width as usize, ppm_float.height as usize, Rgb { red: 0, green: 0, blue: 0 });
    //iterate and change pixels r, g, b values to int
    let width = ppm_int.width;
    for (i, dest_pixel) in ppm_int.iter_row_major_mut().enumerate() {
        let row = i / width;
        let col = i % width;
        let pixel = ppm_float.get(row, col).unwrap();
        // update pixel values as Vec<vec<f64>> to int
        *dest_pixel = Rgb { red: (pixel[0] * denom as f64) as u16, green: (pixel[1] * denom as f64) as u16, blue: (pixel[2] * denom as f64) as u16 };
    }

    ppm_int
}


/// This function takes a 2D array of RGB values and converts them to Y, Pb, Pr values
/// 
/// Arguments:
/// 
/// * `ppm_float`: &mut Array2<Vec<f64>>
pub fn rgb_to_vid_form(ppm_float: &mut Array2<Vec<f64>>) {
    //iterate and change float values to Y, Pb, Pr
    for pixel in ppm_float.iter_row_major_mut() {
        let y = 0.299 * pixel[0] + 0.587 * pixel[1] + 0.114 * pixel[2];
        let pb = -0.168736 * pixel[0] - 0.331264 * pixel[1] + 0.5 * pixel[2];
        let pr = 0.5 * pixel[0] - 0.418688 * pixel[1] - 0.081312 * pixel[2];
        // update video form pixel values at Vec<vec<f64>> to Y, Pb, Pr
        pixel[0] = y;
        pixel[1] = pb;
        pixel[2] = pr;
    }

}

/// > This function takes a 2D array of video form pixel values and converts them to RGB pixel values
/// 
/// Arguments:
/// 
/// * `ppm_float`: &mut Array2<Vec<f64>>
pub fn vid_form_to_rgb(ppm_float: &mut Array2<Vec<f64>>) {
    //iterate and change Y, Pb, Pr values to float
    for pixel in ppm_float.iter_row_major_mut() {
        let r = 1.0 * pixel[0] + 0.0 * pixel[1].clone() + 1.402 * pixel[2];
        let g = 1.0 * pixel[0] - 0.344136 * pixel[1] - 0.714136 * pixel[2];
        let b = 1.0 * pixel[0] + 1.772 * pixel[1] + 0.0 * pixel[2];
        // update video form pixel values at Vec<vec<f64>> to R, G, B
        pixel[0] = r;
        pixel[1] = g;
        pixel[2] = b;
    }

}

/// `array2rgb_to_rgbimg` takes an `Array2<Rgb>` and a `u16` and returns an `RgbImage`
/// 
/// Arguments:
/// 
/// * `ppm_src`: The source image to be converted.
/// * `denom`: the denominator of the RGB values.
/// 
/// Returns:
/// 
/// A RgbImage struct.
pub fn array2rgb_to_rgbimg(ppm_src: &Array2<Rgb>, denom: u16) -> RgbImage {

    let rgb = RgbImage {
        width: ppm_src.width as u32, 
        height: ppm_src.height as u32,
        denominator: denom,
        pixels: ppm_src.data.clone(),
    };
    rgb
}

pub fn vid_form_to_cos_transform(ppm_video_float: &Array2<Vec<f64>>) -> Array2<Vec<f64>> {
    let mut ppm_cos_transform = Array2::new(ppm_video_float.width as usize, ppm_video_float.height as usize, Vec::new());
    // cosine transform
    let width = ppm_video_float.width;
    let height = ppm_video_float.height;

    // iterate through ppm_cost_transform to create a, b, c, d , avg_Pb, and avg_Pr values
    // based on the 2x2 block of pixels from ppm_video_float
     // Check sub-grids
    // Row_start and col_start loops are the coordinates of the top-left corner of the subgrid.
    for row_start in (0..height).step_by(2) {
        for col_start in (0..width).step_by(2) {
            
            // Row_offset and col_offset loops indices the subgrid.
            for row_offset in 0..2 {
                for col_offset in 0..2 {
                    let ppm_src = ppm_video_float.get(row_start + row_offset, col_start + col_offset).unwrap();
                    let ppm_dest = ppm_cos_transform.get_mut(row_start + row_offset, col_start + col_offset).unwrap();
                }
            }
        }
    }

    ppm_cos_transform
}