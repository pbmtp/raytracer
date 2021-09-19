use image::GenericImageView;

use crate::vec3::{Color, Point3};

use super::Texture;

const BYTES_PER_PIXEL: usize = 3;

// Mapped image texture
pub struct ImageTexture {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl ImageTexture {
    pub fn new(filename: &str) -> ImageTexture {
        let img = image::open(filename).expect("Failed to load image");

        let (width, height) = img.dimensions();
        let pixels = img.as_flat_samples_u8().expect("Failed to convert image");
        let mut data = Vec::new();

        data.extend_from_slice(pixels.as_slice());

        ImageTexture {
            data,
            width: width as usize,
            height: height as usize,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        if self.data.is_empty() {
            return Color::new(0.0, 1.0, 1.0);
        }

        // Clamp input texture coordinates to [0,1] x [1,0]
        let uc = u.clamp(0.0, 1.0);
        let vc = 1.0 - v.clamp(0.0, 1.0); // Flip V to image coordinates

        let mut i = (uc * self.width as f64) as usize;
        let mut j = (vc * self.height as f64) as usize;

        // Clamp integer mapping, since actual coordinates should be less than 1.0
        i = i.clamp(0, self.width - 1);
        j = j.clamp(0, self.height - 1);

        let idx = BYTES_PER_PIXEL * i + BYTES_PER_PIXEL * self.width * j;

        let r = self.data[idx] as f64 / 255.0;
        let g = self.data[idx + 1] as f64 / 255.0;
        let b = self.data[idx + 2] as f64 / 255.0;
        Color::new(r, g, b)
    }
}
