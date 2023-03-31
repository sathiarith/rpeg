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
    let ppm_crop: Array2<Rgb> = crop(&ppm);
    let mut ppm_float: Array2<Vec<f64>> = rgb_int_to_float(&ppm_crop, ppm.denominator);
    //print_ppm_as_rgb_array2(&ppm_crop);
    //print_ppm_as_float_array2(&ppm_float);
    //print_ppm_as_rgb_array2(&rgb_float_to_int(&ppm_float, ppm.denominator));
    let ppm_fcrop = rgb_float_to_int(&ppm_float, ppm.denominator);
    // make new rgbimage with new pixels
    let ppm2 = RgbImage {
        width: ppm_crop.width as u32,
        height: ppm_crop.height as u32,
        denominator: ppm.denominator,
        pixels: ppm_fcrop.data,
    };
    // convert ppm_float to video format
    rgb_to_vid_form(&mut ppm_float);
    //iterate through ppm_float and print vector contents
    for row in 0..ppm_float.height {
        for col in 0..ppm_float.width {
            let pixel = ppm_float.get(row,col).unwrap();
            print!("[Y: {}, Pb: {}, Pr: {}]", pixel[0], pixel[1], pixel[2]);
        }
        println!();
    }
    
    let output_path: &str = output;
    ppm2.write(Some(output_path)).unwrap();
}
