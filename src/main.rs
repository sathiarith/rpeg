use std::env;
use rpeg::codec::{compress, decompress};
use rpeg::utilities::*;
extern crate array2;
use array2::Array2;
use csc411_image::{Rgb, RgbImage, Read, Write};



fn main() {
    let args: Vec<String> = env::args().collect();
    let argnum = args.len();
    //assert!(argnum == 2 || argnum == 3);
    let filename = args.iter().nth(2).unwrap();
    let output = args.iter().nth(3).unwrap();
    //read in filename
    let ppm = RgbImage::read(Some(filename)).unwrap();

    // match args[1].as_str() {
    //     "-c" => compress(Some(filename)),
    //     "-d" => decompress(Some(filename)),
    //     _ => {
    //         eprintln!("Usage: rpeg -d [filename]\nrpeg -c [filename]")
    //     }
    // }

    //write out filename to output folder
    let ppm_crop: Array2<Rgb> = trim(&ppm);
    let ppm_float: Array2<ImgRgbasfloat> = rgb_int_to_float(&ppm_crop, ppm.denominator);
    //print_ppm_as_rgb_array2(&ppm_crop);
    //print_ppm_as_float_array2(&ppm_float);
    //print_ppm_as_rgb_array2(&rgb_float_to_int(&ppm_float, ppm.denominator));
    // convert ppm_float to video format
    let ppm_vid_form = rgb_float_to_vid_form(&ppm_float);
    let ppm_rgb_float = vid_form_to_rgb_float(&ppm_vid_form);
    let ppm_fcrop = rgb_float_to_int(&ppm_float, ppm.denominator);
    // make new rgbimage with new pixels

    let ppm2 = array2rgb_to_rgbimg(&ppm_fcrop, ppm.denominator);
    
    let output_path: &str = output;
    ppm2.write(Some(output_path)).unwrap();
}
