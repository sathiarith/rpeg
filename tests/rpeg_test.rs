#[cfg(test)]
mod tests {
    use rpeg::utilities::*;
    extern crate array2;
    use array2::Array2;
    use csc411_image::{Rgb, RgbImage, Read, Write};

    #[test]
    fn trim_t() {
        // Assert that the width and height becomes even
        // file 1
        let filename = "./src/ppm/sample_1280×853.ppm";
        let ppm1 = RgbImage::read(Some(filename)).unwrap();
        assert_eq!(ppm1.width, 1280);
        assert_eq!(ppm1.height, 853);
        let ppm1_trim: Array2<Rgb> = trim(&ppm1);
        let trim1_to_output = array2rgb_to_rgbimg(&ppm1_trim, ppm1.denominator);
        assert_eq!(trim1_to_output.width, 1280);
        assert_eq!(trim1_to_output.height, 852);

        // file 2
        let filename = "./src/ppm/fennec_1731x951.ppm";
        let ppm2 = RgbImage::read(Some(filename)).unwrap();
        assert_eq!(ppm2.width, 1731);
        assert_eq!(ppm2.height, 951);
        let ppm2_trim: Array2<Rgb> = trim(&ppm2);
        let trim2_to_output = array2rgb_to_rgbimg(&ppm2_trim, ppm2.denominator);
        assert_eq!(trim2_to_output.width, 1730);
        assert_eq!(trim2_to_output.height, 950);
    }

    #[test]
    fn rgb_int_to_float_t() {
        let filename = "./src/ppm/sample_640×426.ppm";
        let ppm = RgbImage::read(Some(filename)).unwrap();
        let ppm_trim: Array2<Rgb> = trim(&ppm);
        let ppm_float: Array2<ImgRgbasfloat> = rgb_int_to_float(&ppm_trim, ppm.denominator);
        for (pix_float, pix_rgb) in ppm_float.iter_row_major().zip(ppm_trim.iter_row_major()) {
            assert_eq!(pix_float.red, pix_rgb.red as f64 / ppm.denominator as f64);
            assert_eq!(pix_float.green, pix_rgb.green as f64 / ppm.denominator as f64);
            assert_eq!(pix_float.blue, pix_rgb.blue as f64 / ppm.denominator as f64);
        }

    }
    #[test]
    fn rgb_float_to_int_then_compare_old_vs_new_ints() {
        let filename = "./src/ppm/sample_640×426.ppm";
        let ppm = RgbImage::read(Some(filename)).unwrap();
        let ppm_trim: Array2<Rgb> = trim(&ppm);
        let ppm_float: Array2<ImgRgbasfloat> = rgb_int_to_float(&ppm_trim, ppm.denominator);
        let ppm_rgb: Array2<Rgb> = rgb_float_to_int(&ppm_float, ppm.denominator);
        for (rgb_convert, rgb_orig) in ppm_rgb.iter_row_major().zip(ppm_trim.iter_row_major()) {
            assert_eq!(rgb_convert.red, rgb_orig.red);
            assert_eq!(rgb_convert.green, rgb_orig.green);
            assert_eq!(rgb_convert.blue, rgb_orig.blue);
        }

    }

    #[test]
    fn rgb_float_to_vid_form_t() {
        let filename = "./src/ppm/sample_640×426.ppm";
        let ppm = RgbImage::read(Some(filename)).unwrap();
        let ppm_trim: Array2<Rgb> = trim(&ppm);
        let ppm_float: Array2<ImgRgbasfloat> = rgb_int_to_float(&ppm_trim, ppm.denominator);
        let ppm_vid_form = rgb_float_to_vid_form(&ppm_float);
        for (pix_vid, pix_float) in ppm_vid_form.iter_row_major().zip(ppm_float.iter_row_major()) {
            assert_eq!(pix_vid.y, 0.299 * pix_float.red + 0.587 * pix_float.green + 0.114 * pix_float.blue);
            assert_eq!(pix_vid.pb, -0.168736 * pix_float.red - 0.331264 * pix_float.green + 0.5 * pix_float.blue);
            assert_eq!(pix_vid.pr, 0.5 * pix_float.red - 0.418688 * pix_float.green - 0.081312 * pix_float.blue);
        }
    }
// float approx crate
    #[test]
    fn vid_form_to_rgb_float_then_compare_old_vs_new_floats() {
        let filename = "./src/ppm/sample_640×426.ppm";
        let ppm = RgbImage::read(Some(filename)).unwrap();
        let ppm_trim: Array2<Rgb> = trim(&ppm);
        let ppm_float: Array2<ImgRgbasfloat> = rgb_int_to_float(&ppm_trim, ppm.denominator);
        let ppm_vid_form = rgb_float_to_vid_form(&ppm_float);
        let ppm_vid_form_to_rgb_float = vid_form_to_rgb_float(&ppm_vid_form);
        for (pix_vid_to_float, pix_float) in ppm_vid_form_to_rgb_float.iter_row_major().zip(ppm_float.iter_row_major()) {
            assert_eq!(pix_vid_to_float.red, pix_float.red);
            assert_eq!(pix_vid_to_float.green, pix_float.green);
            assert_eq!(pix_vid_to_float.blue, pix_float.blue);
        }
    }

    // #[test]
    // fn vid_form_to_cos_transform_t() {
    //     let filename = "./src/ppm/sample_640×426.ppm";
    //     let ppm = RgbImage::read(Some(filename)).unwrap();
    //     let ppm_trime: Array2<Rgb> = trime(&ppm);
    //     let ppm_float: Array2<ImageRgbasfloat> = rgb_int_to_float(&ppm_trim, ppm.denominator);
    //     let ppm_vid_form = rgb_float_to_vid_form(&ppm_float);
    //     let ppm_vid_form_to_cos_form = vid_form_to_cos_transform(&ppm_vid_form);
    //     for 

    // }
}