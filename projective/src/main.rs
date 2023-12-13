//! Examples of applying projective transformation to an image.
//!
//! `cargo run --release ./sample.jpg ./output

use image::{error::ImageResult, open, Rgb, ImageBuffer, imageops::crop};
use imageproc::geometric_transformations::{warp_into, Interpolation, Projection};
use std::{env, fs, path::Path};

fn main() -> ImageResult<()> {
    if env::args().len() != 3 {
        panic!("Please enter an input file and a target directory")
    }

    println!("{}", env::args().nth(0).unwrap());  // target\release\projection.exe

    let input_path = env::args().nth(1).unwrap();  // input image
    let output_dir = env::args().nth(2).unwrap();  // output directory

    let input_path = Path::new(&input_path);
    let output_dir = Path::new(&output_dir);

    if !output_dir.is_dir() {
        fs::create_dir(output_dir).expect("Failed to create output directory")
    }

    if !input_path.is_file() {
        panic!("Input file does not exist");
    }

    let image = open(input_path)
        .expect(&format!("Could not load image at {:?}", input_path))
        .to_rgb8();

    let out_width: u32 = 64*9;
    let out_height: u32 = 64*9;

    // Crop points of input image
    let p0: (f32, f32) = (327.0, 100.0);  // Upper left
    let p1: (f32, f32) = (939.0, 144.0);  // Upper right
    let p2: (f32, f32) = (1049.0, 810.0);  // Lower right
    let p3: (f32, f32) = (336.0, 887.0);  // Lower left

    let mut extracted_image = ImageBuffer::new(out_width, out_height);
    let h = Projection::from_control_points(
        [p0, p1, p2, p3],
        [(0.0, 0.0),
            (out_width as f32, 0.0),
            (out_width as f32, out_height as f32),
            (0.0, out_height as f32)]
        ).unwrap();
    println!( "{:?}", &h);

    warp_into(
        &image,
        &h,
        Interpolation::Bilinear,
        Rgb([255, 0, 0]),
        &mut extracted_image,
    );
    extracted_image.save(output_dir.join("extracted.png"))?;

    // Splitting an image
    let mut n: u8 = 0;
    for col in 0..9 {  // Vertical
        for row in 0..9 {  // Horizontal
            let cell_iamge = crop(&mut extracted_image, row*64, col*64, 64, 64);
            cell_iamge.to_image().save(output_dir.join("cell".to_owned() + &n.to_string() +".png")).unwrap();
            println!("{}: {}, {}", n, col, row);
            n += 1 ;
        }
     }

    Ok(())
}

