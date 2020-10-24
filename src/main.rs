#![allow(dead_code)]

extern crate image;
extern crate indicatif;

use image::{ImageBuffer, Rgb};
use indicatif::ProgressBar;

mod ray;
mod vec3;

use ray::Ray;
use vec3::{Color, Point3, Vec3};

// Size
const RATIO: f32 = 16.0 / 9.0;
const WIDTH: u32 = 800;
const HEIGHT: u32 = (WIDTH as f32 / RATIO) as u32;

fn render_simple(name: &str) {
    // create image buffer
    let mut imgbuf = ImageBuffer::new(WIDTH, HEIGHT);

    // Iterate over the coordinates and pixels of the image
    let len = WIDTH as u64 * HEIGHT as u64;
    let bar = ProgressBar::new(len);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        bar.inc(1);

        let c = Color::from((
            x as f32 / (WIDTH as f32 - 1f32),
            y as f32 / (HEIGHT as f32 - 1f32),
            0.25f32,
        ));
        *pixel = Rgb(c.to_u8());
    }
    bar.finish();

    // write the generated image (format is deduced based on extension)
    imgbuf.save(name).unwrap();
}

fn hit_sphere(center: Point3, radius: f32, r: &Ray) -> f32 {
    let oc: Vec3 = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = oc.dot(&r.direction());
    let c = oc.length_squared() - radius.powi(2);
    let discriminant = half_b.powi(2) - a * c;
    
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(r: Ray) -> Color {
    let mut t = hit_sphere(Point3::new(0.0,0.0,-1.0), 0.5, &r);
    if t > 0.0f32 {
        let v = r.point_at(t) - Vec3::new(0.0, 0.0, -1.0);
        let n = v.to_unit_vector();

        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let unit_direction = r.direction().to_unit_vector();
    t = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn render_with_ray(name: &str) {
    // Camera
    let viewport_height = 2.0f32;
    let viewport_width = RATIO * viewport_height;
    let focal_length = 1.0f32;

    let origin = Point3::zero();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // create image buffer
    let mut imgbuf = ImageBuffer::new(WIDTH, HEIGHT);

    // Iterate over the coordinates and pixels of the image
    let len = WIDTH as u64 * HEIGHT as u64;
    let bar = ProgressBar::new(len);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        bar.inc(1);

        let u = x as f32 / (WIDTH as f32 - 1f32);
        let v = y as f32 / (HEIGHT as f32 - 1f32);
        let r = Ray::new(
            origin,
            lower_left_corner + u * horizontal + v * vertical - origin,
        );

        let c = ray_color(r);

        *pixel = Rgb(c.to_u8());
    }
    bar.finish();

    // write the generated image (format is deduced based on extension)
    imgbuf.save(name).unwrap();
}

fn main() {
    // render_simple("out.png");
    render_with_ray("out.png");
}
