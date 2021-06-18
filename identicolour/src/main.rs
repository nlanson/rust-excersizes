//! An example of generating julia fractals.
extern crate image;
extern crate num_complex;
use md5;

mod Identicon {
    pub struct Identicon {
        pub input: String,
        pub hash: md5::Digest,
        pub rgb: [u8; 3]
    }

    impl Identicon {
        pub fn new(input: String) -> Identicon {
            let input_clone: String = input.clone();
            let hash: md5::Digest = md5::compute(input);
            let rgb: [u8; 3] = super::extractIntsFromHash(&hash);
            
            return Identicon {
                input: input_clone,
                hash: hash,
                rgb: rgb
            }
        }
    }
}



fn main() {
    let input: String = String::from("0xacF1602162436041cEE343c4Ba34526F4AD6F236");
    let identicon: Identicon::Identicon = Identicon::Identicon::new(input);
    println!("Your identicolour is {}, {}, {}", identicon.rgb[0], identicon.rgb[1], identicon.rgb[2]);


    //Define image dimensions
    let imgx = 64;
    let imgy = 64;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = identicon.rgb[0];
        let g = identicon.rgb[1];
        let b = identicon.rgb[2];
        *pixel = image::Rgb([r, g, b]);
    }

    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        for y in 0..imgy {
           
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("./identicolour.png").unwrap();
}

fn extractIntsFromHash(hash: &md5::Digest) -> [u8; 3] {
    [hash[0], hash[1], hash[2]]
}