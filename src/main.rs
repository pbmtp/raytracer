#![allow(dead_code)]

extern crate image;
extern crate indicatif;

use image::{ImageBuffer, Rgb};
use indicatif::ProgressBar;

mod vec3;

use vec3::Vec3;

// type alias
type Point3 = Vec3;
type Color = Vec3;

// Size
const RATIO: f32 = 16.0 / 9.0;
const WIDTH: u32 = 800;
const HEIGHT: u32 = (WIDTH as f32 / RATIO) as u32;

fn render_png(name: &str) {
    // create image buffer
    let mut imgbuf = ImageBuffer::new(WIDTH, HEIGHT);

    // Iterate over the coordinates and pixels of the image
    let len = WIDTH as u64 * HEIGHT as u64;
    let bar = ProgressBar::new(len);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        bar.inc(1);

        let c = Color::from((x as f32 / (WIDTH as f32 - 1f32), y as f32 / (HEIGHT as f32 - 1f32), 0.25f32));
        *pixel = Rgb(c.to_u8());
    }
    bar.finish();

    // write the generated image (format is deduced based on extension)
    imgbuf.save(name).unwrap();
}

fn main() {
    render_png("out.png");
}
