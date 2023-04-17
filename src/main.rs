use std::env;
use rpeg::codec::{compress, decompress};
extern crate array2;
extern crate bitpack;
use csc411_image::{Rgb, RgbImage, Read, Write};



fn main() {
    let args: Vec<String> = env::args().collect();
    let arg_num = args.len();
    assert!(arg_num == 2 || arg_num == 3);
    let filename = args.iter().nth(2).unwrap();
    //let output = args.iter().nth(3).unwrap();
    

    match args[1].as_str() {
        "-c" => compress(Some(filename)),
        "-d" => decompress(Some(filename), 255),
        _ => {
            eprintln!("Usage: rpeg -d [filename]\nrpeg -c [filename]")
        }
    }

    // let ppm_trim: Array2<Rgb> = trim(&ppm);
    // let ppm_float: Array2<ImgRgbasfloat> = rgb_int_to_float(&ppm_trim, ppm.denominator);
    // let ppm_vid_form = rgb_float_to_vid_form(&ppm_float);
    // let ppm_discrete_cos_form = vid_form_to_cos_transform(&ppm_vid_form);
    // let ppm_cos_form_to_quantized = cos_form_to_quantize(&ppm_discrete_cos_form);
    // let ppm_quantized_to_cos_form_decomp = quantize_to_cos_form(&ppm_cos_form_to_quantized);
    // let ppm_vid_form_decomp = cos_transform_to_vid_form(&ppm_quantized_to_cos_form_decomp);
    // let ppm_rgb_float_decomp = vid_form_to_rgb_float(&ppm_vid_form_decomp);
    // let ppm_rgb_int_decomp = rgb_float_to_int(&ppm_rgb_float_decomp, ppm.denominator);
    // let ppm2 = array2rgb_to_rgbimg(&ppm_rgb_int_decomp, ppm.denominator);

    //let output_path: &str = output;
    //ppm2.write(Some(output_path)).unwrap();
}
