extern crate array2;
extern crate bitpack;
use array2::Array2;
use csc411_image::{Rgb, RgbImage, Read, Write};
use csc411_rpegio::*;
use crate::utilities::*;

pub fn compress(filename: Option<&str>) {
    //read in filename
    let ppm = RgbImage::read(filename).unwrap();
    let ppm_trim: Array2<Rgb> = trim(&ppm);
    let ppm_float: Array2<ImgRgbasfloat> = rgb_int_to_float(&ppm_trim, ppm.denominator);
    let ppm_vid_form = rgb_float_to_vid_form(&ppm_float);
    let ppm_discrete_cos_form = vid_form_to_cos_transform(&ppm_vid_form);
    let ppm_cos_form_to_quantized = cos_form_to_quantize(&ppm_discrete_cos_form);
    let ppm_packed = pack(&ppm_cos_form_to_quantized);
    let compressed_data: Vec<[u8; 4]> = ppm_packed.into_iter().map(u32::to_be_bytes).collect();
    //println!("Compressed data: {:?}", compressed_data);
    
    output_rpeg_data(&compressed_data, ppm_trim.width, ppm_trim.height).unwrap();
}

pub fn decompress(filename: Option<&str>, denom: u16) {


    // Read in the rpeg data
    let (compressed_data, width, height) = csc411_rpegio::input_rpeg_data(filename).unwrap();

    // // You will likely want to interpret the `[u8; 4]`'s as big-endian `u32`s
    let words: Vec<u32> = compressed_data.into_iter().map(u32::from_be_bytes).collect();
    let words_to_quantized = unpack(&words, width, height);
    let ppm_quantized_to_cos_form_decomp = quantize_to_cos_form(&words_to_quantized);
    let ppm_vid_form_decomp = cos_transform_to_vid_form(&ppm_quantized_to_cos_form_decomp);
    let ppm_rgb_float_decomp = vid_form_to_rgb_float(&ppm_vid_form_decomp);
    let ppm_rgb_int_decomp = rgb_float_to_int(&ppm_rgb_float_decomp, denom);
    let ppm2 = array2rgb_to_rgbimg(&ppm_rgb_int_decomp, denom);

    let output = "./src/output/out.ppm";
    let output_path: &str = output;
    ppm2.write(Some(output_path)).unwrap();
}
