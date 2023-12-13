use std::env;
use image::GenericImageView;

fn main() {
    let path = env::current_dir();
    println!("{:?}", path);

    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    // let img = image::open("C:Users\admin\Documents\Rust\image\sample.png").unwrap();
    let img = image::open("sample.jpg").unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());

    // Write the contents of this image to the Writer in PNG format.
    img.save("test.png").unwrap();
}