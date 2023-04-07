extern crate array2;
extern crate bitpack;
use array2::Array2;
use bitpack::bitpack::*;
use csc411_image::{Rgb, RgbImage, Read, Write};
use crate::utilities::*;

pub fn compress(filename: Option<&str>) {
    //read in filename
    let ppm = RgbImage::read(filename).unwrap();
    let ppm_trim: Array2<Rgb> = trim(&ppm);
    let ppm_float: Array2<ImgRgbasfloat> = rgb_int_to_float(&ppm_trim, ppm.denominator);
    let ppm_vid_form = rgb_float_to_vid_form(&ppm_float);
    let ppm_discrete_cos_form = vid_form_to_cos_transform(&ppm_vid_form);
    let ppm_cos_form_to_quantized = cos_form_to_quantize(&ppm_discrete_cos_form);
}

pub fn decompress(filename: Option<&str>) {
    let ppm_quantized_to_cos_form_decomp = quantize_to_cos_form(&ppm_cos_form_to_quantized);
    let ppm_vid_form_decomp = cos_transform_to_vid_form(&ppm_quantized_to_cos_form_decomp);
    let ppm_rgb_float_decomp = vid_form_to_rgb_float(&ppm_vid_form_decomp);
    let ppm_rgb_int_decomp = rgb_float_to_int(&ppm_rgb_float_decomp, ppm.denominator);
    let ppm2 = array2rgb_to_rgbimg(&ppm_rgb_int_decomp, ppm.denominator);
}
