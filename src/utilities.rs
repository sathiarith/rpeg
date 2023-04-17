extern crate array2;
use bitpack::bitpack::*;
use array2::Array2;
use csc411_image::{Rgb, RgbImage};
use csc411_arith::{chroma_of_index, index_of_chroma};

// Struct for pixels
#[derive(Debug, Clone, Copy)]
pub struct ImgRgbasfloat {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

// struct for video format
#[derive(Debug, Clone, Copy)]
pub struct ImgVidForm {
    pub y: f64,
    pub pb: f64,
    pub pr: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Quartet {
    pub y1: f64,
    pub y2: f64,
    pub y3: f64,
    pub y4: f64,
    pub pb1: f64,
    pub pr1: f64,
    pub pb2: f64,
    pub pr2: f64,
    pub pb3: f64,
    pub pr3: f64,
    pub pb4: f64,
    pub pr4: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct ImgCosForm {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub avg_pb: f64,
    pub avg_pr: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct ImgQuantizeForm {
    pub a: u64,
    pub b: i64,
    pub c: i64,
    pub d: i64,
    pub avg_pb: i64,
    pub avg_pr: i64,
}


/// It takes an image, crops it to be even in width and height, and returns the cropped image
/// 
/// Arguments:
/// 
/// * `src`: The source image to crop
/// 
/// Returns:
/// 
/// A 2D array of RGB values
pub fn trim(src: &RgbImage) -> Array2<Rgb> {
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
    // println!("Width: {}, Height: {}", src.width, src.height);
    // print width height of dest
    // println!("DWidth: {}, DHeight: {}", dest.width, dest.height);
    let _dest_height = dest.height;
    let dest_width = dest.width;
    // println!("Length of data: {}", dest.data.len());
    for (i, dest_pixel) in dest.iter_row_major_mut().enumerate() {
        let row = i / dest_width;
        let col = i % dest_width;
        //println!("Index: {}, Row: {}, Col: {}", row*dest_width+col, row, col);
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
pub fn print_ppm_as_float_array2(ppm_crop: &Array2<Rgb>) {
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
pub fn rgb_int_to_float(ppm_crop: &Array2<Rgb>, denom: u16) -> Array2<ImgRgbasfloat> {
    let mut ppm_float = Array2::new(ppm_crop.width as usize, ppm_crop.height as usize, ImgRgbasfloat { red: 0.0, green: 0.0, blue: 0.0 });
    //iterate and change pixels r, g, b values to float
    let width = ppm_float.width;
    for (i, dest_pixel) in ppm_float.iter_row_major_mut().enumerate() {
        let row = i / width;
        let col = i % width;
        let pixel = ppm_crop.get(row, col).unwrap();
        // update pixel values as float to Vec<vec<f64>>
        *dest_pixel = ImgRgbasfloat { red: pixel.red as f64 / denom as f64, green: pixel.green as f64 / denom as f64, blue: pixel.blue as f64 / denom as f64 };
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
pub fn rgb_float_to_int(ppm_float: &Array2<ImgRgbasfloat>, denom: u16) -> Array2<Rgb> {
    let mut ppm_int = Array2::new(ppm_float.width as usize, ppm_float.height as usize, Rgb { red: 0, green: 0, blue: 0 });
    //iterate and change pixels r, g, b values to int
    let width = ppm_int.width;
    for (i, dest_pixel) in ppm_int.iter_row_major_mut().enumerate() {
        let row = i / width;
        let col = i % width;
        let pixel = ppm_float.get(row, col).unwrap();
        // update pixel values as Vec<vec<f64>> to int
        *dest_pixel = Rgb { red: (pixel.red * denom as f64) as u16, green: (pixel.green * denom as f64) as u16, blue: (pixel.blue * denom as f64) as u16 };
    }

    ppm_int
}


/// This function takes a 2D array of RGB values and converts them to Y, Pb, Pr values
/// 
/// Arguments:
/// 
/// * `ppm_float`: &mut Array2<Vec<f64>>
pub fn rgb_float_to_vid_form(ppm_float: &Array2<ImgRgbasfloat>) -> Array2<ImgVidForm> {
    let mut ppm_vid_form = Array2::new(ppm_float.width as usize, ppm_float.height as usize, ImgVidForm { y: 0.0, pb: 0.0, pr: 0.0 });
    //iterate and change float values to Y, Pb, Pr
    // for pixel in ppm_vid_form.iter_row_major_mut() {
    //     let y = 0.299 * pixel.red + 0.587 * pixel.green + 0.114 * pixel.blue;
    //     let pb = -0.168736 * pixel.red - 0.331264 * pixel.green + 0.5 * pixel.blue;
    //     let pr = 0.5 * pixel.red - 0.418688 * pixel.green - 0.081312 * pixel.blue;
    //     // update video form pixel values at Vec<vec<f64>> to Y, Pb, Pr
    //     pixel[0] = y;
    //     pixel[1] = pb;
    //     pixel[2] = pr;
    //     ppm_vid_form = 
    // }
    let width = ppm_vid_form.width;
    for (i, dest_pixel) in ppm_vid_form.iter_row_major_mut().enumerate() {
        let row = i / width;
        let col = i % width;
        let pixel = ppm_float.get(row, col).unwrap();
        // update pixels
        *dest_pixel = ImgVidForm { 
            y: 0.299 * pixel.red + 0.587 * pixel.green + 0.114 * pixel.blue, 
            pb: -0.168736 * pixel.red - 0.331264 * pixel.green + 0.5 * pixel.blue, 
            pr: 0.5 * pixel.red - 0.418688 * pixel.green - 0.081312 * pixel.blue, 
        };
    }

    ppm_vid_form
}

/// > This function takes a 2D array of video form pixel values and converts them to RGB pixel values
/// 
/// Arguments:
/// 
/// * `ppm_float`: &mut Array2<Vec<f64>>
pub fn vid_form_to_rgb_float(ppm_vid_form: &Array2<ImgVidForm>) -> Array2<ImgRgbasfloat>{
    let mut ppm_rgb = Array2::new(ppm_vid_form.width as usize, ppm_vid_form.height as usize, ImgRgbasfloat { red: 0.0, green: 0.0, blue: 0.0 });
    //iterate and change Y, Pb, Pr values to float
    // for pixel in ppm_float.iter_row_major_mut() {
    //     let r = 1.0 * pixel[0] + 0.0 * pixel[1] + 1.402 * pixel[2];
    //     let g = 1.0 * pixel[0] - 0.344136 * pixel[1] - 0.714136 * pixel[2];
    //     let b = 1.0 * pixel[0] + 1.772 * pixel[1] + 0.0 * pixel[2];
    //     // update video form pixel values at Vec<vec<f64>> to R, G, B
    //     pixel[0] = r;
    //     pixel[1] = g;
    //     pixel[2] = b;
    // }
    let width = ppm_rgb.width;
    for (i, dest_pixel) in ppm_rgb.iter_row_major_mut().enumerate() {
        let row = i / width;
        let col = i % width;
        let pixel = ppm_vid_form.get(row, col).unwrap();
        // update pixels
        *dest_pixel = ImgRgbasfloat { 
            red: 1.0 * pixel.y + 0.0 * pixel.pb + 1.402 * pixel.pr, 
            green: 1.0 * pixel.y - 0.344136 * pixel.pb - 0.714136 * pixel.pr, 
            blue: 1.0 * pixel.y + 1.772 * pixel.pb + 0.0 * pixel.pr, 
        };
    }
    ppm_rgb
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

// helper functions for quantization
static COSINE_FORCE: f32 = 0.3;
fn scale_sat(x: f32, max_magnitude: f32) -> f32 {
    if x > max_magnitude {
        max_magnitude
    } else if x < -max_magnitude {
        -max_magnitude
    } else {
        x / max_magnitude
    }
}

fn reverse_scale_sat(x: f32, max_magnitude: f32) -> f32 {

    x * max_magnitude / 1000.0
}

fn smax(bits: i32) -> i32 {
    return (1 << bits) / 2 - 1;
}

pub fn vid_form_to_cos_transform(ppm_video_float: &Array2<ImgVidForm>) -> Array2<ImgCosForm> {
    
    //let mut index = (scale_sat(b, COSINE_FORCE) * smax(5) as f32).floor();

    let mut ppm_cos_transform = Array2::new(ppm_video_float.width as usize, ppm_video_float.height as usize, 
        ImgCosForm { 
            a: 0.0, 
            b: 0.0, 
            c: 0.0, 
            d: 0.0, 
            avg_pb: 0.0, 
            avg_pr: 0.0 });
    // cosine transform
    let width = ppm_video_float.width;
    let height = ppm_video_float.height;

    // iterate through ppm_cost_transform to create a, b, c, d , avg_Pb, and avg_Pr values
    // based on the 2x2 block of pixels from ppm_video_float
    // Check sub-grids
    // Row_start and col_start loops are the coordinates of the top-left corner of the subgrid.
    for row_start in (0..height).step_by(2) {
        for col_start in (0..width).step_by(2) {
            
            let mut block_2by2 = Quartet {
                y1: 0.0,
                y2: 0.0,
                y3: 0.0,
                y4: 0.0,
                pb1: 0.0,
                pr1: 0.0,
                pb2: 0.0,
                pr2: 0.0,
                pb3: 0.0,
                pr3: 0.0,
                pb4: 0.0,
                pr4: 0.0,
            }; 
            // Row_offset and col_offset loops indices of the subgrid.
            // Populate the values of the subgrid
            for row_offset in 0..2 {
                for col_offset in 0..2 {
                    // Retrieve y, pb, pr values from ppm_video_float
                    if row_offset == 0 && col_offset == 0 {
                        let ppm_src_ul = ppm_video_float.get(row_start + row_offset, col_start + col_offset).unwrap();
                        block_2by2.y1 = ppm_src_ul.y;
                        block_2by2.pb1 = ppm_src_ul.pb;
                        block_2by2.pr1 = ppm_src_ul.pr;
                    } else if row_offset == 0 && col_offset == 1 {
                        let ppm_src_ur = ppm_video_float.get(row_start + row_offset, col_start + col_offset).unwrap();
                        block_2by2.y2 = ppm_src_ur.y;
                        block_2by2.pb2 = ppm_src_ur.pb;
                        block_2by2.pr2 = ppm_src_ur.pr;
                    } else if row_offset == 1 && col_offset == 0 {
                        let ppm_src_ll = ppm_video_float.get(row_start + row_offset, col_start + col_offset).unwrap();
                        block_2by2.y3 = ppm_src_ll.y;
                        block_2by2.pb3 = ppm_src_ll.pb;
                        block_2by2.pr3 = ppm_src_ll.pr;
                    } else if row_offset == 1 && col_offset == 1 {
                        let ppm_src_lr = ppm_video_float.get(row_start + row_offset, col_start + col_offset).unwrap();
                        block_2by2.y4 = ppm_src_lr.y;
                        block_2by2.pb4 = ppm_src_lr.pb;
                        block_2by2.pr4 = ppm_src_lr.pr;
                    } 
                }
            }

            // Get the destination pixel
            for row_offset in 0..2 {
                for col_offset in 0..2 {
                    let ppm_dest: &mut ImgCosForm = ppm_cos_transform.get_mut(row_start + row_offset, col_start + col_offset).unwrap();
                    // a = (Y4 + Y3 + Y2 + Y1)/4.0
                    // b = (Y4 + Y3 − Y2 − Y1)/4.0
                    // c = (Y4 − Y3 + Y2 − Y1)/4.0
                    // d = (Y4 − Y3 − Y2 + Y1)/4.0
                    
                    let a = (block_2by2.y4 + block_2by2.y3 + block_2by2.y2 + block_2by2.y1)/4.0;
                    let b = (block_2by2.y4 + block_2by2.y3 - block_2by2.y2 - block_2by2.y1)/4.0;
                    let c = (block_2by2.y4 - block_2by2.y3 + block_2by2.y2 - block_2by2.y1)/4.0;
                    let d = (block_2by2.y4 - block_2by2.y3 - block_2by2.y2 + block_2by2.y1)/4.0;
                    
                    let avg_pb = (block_2by2.pb1 + block_2by2.pb2 + block_2by2.pb3 + block_2by2.pb4)/4.0;
                    
                    let avg_pr = (block_2by2.pr1 + block_2by2.pr2 + block_2by2.pr3 + block_2by2.pr4)/4.0;

                    *ppm_dest = ImgCosForm {
                        a: a,
                        b: b,
                        c: c,
                        d: d,
                        avg_pb: avg_pb,
                        avg_pr: avg_pr,
                    };
                }
            }
            
        }
    }

    ppm_cos_transform
}

pub fn cos_transform_to_vid_form(ppm_cos_form: &Array2<ImgCosForm>) -> Array2<ImgVidForm> {
    let mut ppm_vid_form = Array2::new(ppm_cos_form.width as usize, ppm_cos_form.height as usize, 
        ImgVidForm {
            y: 0.0,
            pb: 0.0,
            pr: 0.0,});
    
    let width = ppm_cos_form.width;
    let height = ppm_cos_form.height;

    for row_start in (0..height).step_by(2) {
        for col_start in (0..width).step_by(2) {
            
            // Row_offset and col_offset loops indices of the subgrid.
            // Populate the values of the subgrid
            for row_offset in 0..2 {
                for col_offset in 0..2 {
                    let ppm_dest = ppm_vid_form.get_mut(row_start + row_offset, col_start + col_offset).unwrap();
                    // Retrieve y, pb, pr values from ppm_cos_form
                    // Y1 = a − b − c + d
                    // Y2 = a − b + c − d
                    // Y3 = a + b − c − d
                    // Y4 = a + b + c + d
                    if row_offset == 0 && col_offset == 0 {
                        let ppm_src_ul = ppm_cos_form.get(row_start + row_offset, col_start + col_offset).unwrap();
                        
                        *ppm_dest = ImgVidForm {
                            y: ppm_src_ul.a - ppm_src_ul.b - ppm_src_ul.c + ppm_src_ul.d,
                            pb: ppm_src_ul.avg_pb,
                            pr: ppm_src_ul.avg_pr,
                        };
                    } else if row_offset == 0 && col_offset == 1 {
                        let ppm_src_ur = ppm_cos_form.get(row_start + row_offset, col_start + col_offset).unwrap();
                        
                        *ppm_dest = ImgVidForm {
                            y: ppm_src_ur.a - ppm_src_ur.b + ppm_src_ur.c - ppm_src_ur.d,
                            pb: ppm_src_ur.avg_pb,
                            pr: ppm_src_ur.avg_pr,
                        };
                    } else if row_offset == 1 && col_offset == 0 {
                        let ppm_src_ll = ppm_cos_form.get(row_start + row_offset, col_start + col_offset).unwrap();
                        
                        *ppm_dest = ImgVidForm {
                            y: ppm_src_ll.a + ppm_src_ll.b - ppm_src_ll.c - ppm_src_ll.d,
                            pb: ppm_src_ll.avg_pb,
                            pr: ppm_src_ll.avg_pr,
                        };
                    } else if row_offset == 1 && col_offset == 1 {
                        let ppm_src_lr = ppm_cos_form.get(row_start + row_offset, col_start + col_offset).unwrap();
                        
                        *ppm_dest = ImgVidForm {
                            y: ppm_src_lr.a + ppm_src_lr.b + ppm_src_lr.c + ppm_src_lr.d,
                            pb: ppm_src_lr.avg_pb,
                            pr: ppm_src_lr.avg_pr,
                        };
                    } else {
                        unreachable!();
                    }
                }
            }
            
        }
    }
    ppm_vid_form
}

pub fn cos_form_to_quantize(ppm_cos_form: &Array2<ImgCosForm>) -> Array2<ImgQuantizeForm> {
    let mut ppm_quantized = Array2::new(ppm_cos_form.width as usize, ppm_cos_form.height as usize, 
        ImgQuantizeForm {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            avg_pb: 0,
            avg_pr: 0,});
    
        for (pix_quantize, pix_cos) in ppm_quantized.iter_row_major_mut().zip(ppm_cos_form.iter_row_major()) {
            
            let index_b = (scale_sat(pix_cos.b as f32, COSINE_FORCE) * smax(5) as f32).floor();
            // println!("index_b: {}", pix_cos.b);
            // println!("scale_sat: {}", scale_sat(pix_cos.b as f32, COSINE_FORCE));
            // println!("smax: {}", smax(5));
            // println!("index_b: {}", index_b);
            /* scale_sat is some number between -.5 to .5 and we are straining it to at max -.3 or .3. We then force this float number to be represented
            as 5-bits (i32). Within smax, we reserve the left-most bit for the negative value. Therefore the actual bit representation is
            max negative number and some 4 bit number (e.g., 4 1 bits = 15 vs. 4 0 bits; (2^bits)-1). The range would be -16 + 15 = -1 or -16 + 0 = -16. This
            is for the half that the entire 5 signed bits has left-most value of 1. When the left-most bit is 0, then the range is 0 + 15 = 15 to
            0 + 0 = 0. In total, -16 -> -1 -> 0 -> 15. Given pix_cos.b = .27 into scale_sat, returning .9; .9 * 15 (i.e., 5 bits), 
            transformation is 13.5 rounded up is 14. So the bit represenation (not pix value) is 14 or 0b01110 */
            let index_c = (scale_sat(pix_cos.c as f32, COSINE_FORCE) * smax(5) as f32).floor();
            /* Given pix_cos.c = -0.27 into scale_sat, returning -0.9; -0.9 * 15 = -13.5 rounded down is -14. 
            So the bit represenation (not pix value) is -14 or 0b10010 */
            let index_d = (scale_sat(pix_cos.d as f32, COSINE_FORCE) * smax(5) as f32).floor();
            let index_avg_pb = index_of_chroma(pix_cos.avg_pb as f32);
            let index_avg_pr = index_of_chroma(pix_cos.avg_pr as f32);
            *pix_quantize = ImgQuantizeForm {
                /* Given pix_cos.a = .8 (keep in mind, a is unsigned), we force the float into 9-bits ((2^9)-1) = 511). The result is
                .8 * 511 = 408.8 and when we round up, 409 or 110011001. */
                a: (pix_cos.a * smax(9) as f64).round() as u64,
                b: index_b as i64,
                c: index_c as i64,
                d: index_d as i64,
                avg_pb: index_avg_pb as i64,
                avg_pr: index_avg_pr as i64,
            };
            //print pix_quantize
            //println!("pix_quantize: {:?}", pix_quantize);
        }

    ppm_quantized
}

pub fn quantize_to_cos_form(ppm_quantized: &Array2<ImgQuantizeForm>) -> Array2<ImgCosForm> {
    let mut ppm_cos_form = Array2::new(ppm_quantized.width as usize, ppm_quantized.height as usize, 
        ImgCosForm {
            a: 0.0,
            b: 0.0,
            c: 0.0,
            d: 0.0,
            avg_pb: 0.0,
            avg_pr: 0.0,});
    
        for (pix_cos, pix_quantize) in ppm_cos_form.iter_row_major_mut().zip(ppm_quantized.iter_row_major()) {
            
            // If scale_sat_b * 15 = pix_quantize.b then:
            // scale_sat_b = pix_quantize.b / 15
            let scale_sat_b = pix_quantize.b * 1000 / smax(5) as i64;
            let scale_sat_c = pix_quantize.c * 1000 / smax(5) as i64;
            let scale_sat_d = pix_quantize.d * 1000 / smax(5) as i64;
            // println!("pix_quantize_c: {}", pix_quantize.c);
            // println!("scale_sat_c: {}", scale_sat_c);
            // scale_b * .3 = pix_cos.b
            let b = reverse_scale_sat(scale_sat_b as f32, COSINE_FORCE) as f64;
            let c = reverse_scale_sat(scale_sat_c as f32, COSINE_FORCE) as f64;
            let d = reverse_scale_sat(scale_sat_d as f32, COSINE_FORCE) as f64;
            
            *pix_cos = ImgCosForm {
                a: (pix_quantize.a * 1000 / smax(9) as u64) as f64 / 1000.0,
                b: b,
                c: c,
                d: d,
                avg_pb: chroma_of_index(pix_quantize.avg_pb as usize) as f64,
                avg_pr: chroma_of_index(pix_quantize.avg_pr as usize) as f64,
            };
            //println!("pix_cos: {:?}", pix_cos);
        }

    ppm_cos_form
    
}

pub fn word_u32_prep(a: u64, b: i64, c: i64, d: i64, pb: u64, pr: u64) -> u32 {
    let mut word: Option<u64> = Some(0);
    word = newu(word.unwrap(), 9, 23, a);
    word = news(word.unwrap(), 5, 18, b);
    word = news(word.unwrap(), 5, 13, c);
    word = news(word.unwrap(), 5, 8, d);
    word = newu(word.unwrap(), 4, 4, pb);
    word = newu(word.unwrap(), 4, 0, pr);

    return word.unwrap() as u32;
}

pub fn pack(ppm_quantized: &Array2<ImgQuantizeForm>) -> Vec<u32> {
    let mut ppm_packed = Vec::new();
    let width = ppm_quantized.width;
    let height = ppm_quantized.height;

    for row_start in (0..height).step_by(2) {
        for col_start in (0..width).step_by(2) {
            for row_offset in 0..2 {
                for col_offset in 0..2 {
                    if col_offset == 0 && row_offset == 0 {
                        let ppm_src = ppm_quantized.get(row_start + row_offset, col_start + col_offset).unwrap();
                        let word = word_u32_prep(ppm_src.a, ppm_src.b, ppm_src.c, ppm_src.d, ppm_src.avg_pb as u64, ppm_src.avg_pr as u64);
                        ppm_packed.push(word);
                    }
                }
            }
        }
    }

    ppm_packed
}


pub fn unpack(rpeg_data: &Vec<u32>, ppm_width: usize, ppm_height: usize) -> Array2<ImgQuantizeForm> {
    let mut ppm_quantized = Array2::new(ppm_width, ppm_height, ImgQuantizeForm {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        avg_pb: 0,
        avg_pr: 0,
    });
    for word in rpeg_data.iter() {
        let a = getu(*word as u64, 9, 23);
        let b = gets(*word as u64, 5, 18);
        let c = gets(*word as u64, 5, 13);
        let d = gets(*word as u64, 5, 8);
        let avg_pb = getu(*word as u64, 4, 4);
        let avg_pr = getu(*word as u64, 4, 0);
        for row_start in (0..ppm_height).step_by(2) {
            for col_start in (0..ppm_width).step_by(2) {
                for row_offset in 0..2 {
                    for col_offset in 0..2 {
                        let ppm_dest = ppm_quantized.get_mut(row_start + row_offset, col_start + col_offset).unwrap();
                        *ppm_dest = ImgQuantizeForm {
                            a: a,
                            b: b,
                            c: c,
                            d: d,
                            avg_pb: avg_pb as i64,
                            avg_pr: avg_pr as i64,
                        };
                    }
                }
            }
        }
    }
    ppm_quantized
}