#![feature(clamp)]
#![allow(dead_code)]

// https://raytracing.github.io/books/RayTracingInOneWeekend.html

extern crate image;
extern crate indicatif;
extern crate rand;

use image::{ImageBuffer, Rgb};
use indicatif::ProgressBar;

mod camera;
mod hittable;
mod hittable_list;
mod materials;
mod ray;
mod sphere;
mod tools;
mod vec3;

use camera::Camera;
use hittable::Hittable;
use hittable_list::HittableList;
use materials::{Lambertian, Metal};
use ray::Ray;
use sphere::Sphere;
use tools::random_double;
use vec3::{Color, Point3};

// Size
const RATIO: f64 = 16.0 / 9.0;
const WIDTH: u32 = 800;
const HEIGHT: u32 = (WIDTH as f64 / RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_DEPTH: u32 = 50;

// Chapter 9 https://raytracing.github.io/books/RayTracingInOneWeekend.html#metal
fn ray_color_ch9<T: Hittable>(r: &Ray, world: &T, depth: u32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return Color::zero();
    }

    if let Some(hr) = world.hit(&r, 0.001, std::f64::INFINITY) {
        let scatter = hr.material.scatter(&r, &hr);
        if let Some(r) = scatter.scattered {
            return scatter.attenuation * ray_color_ch9(&r, world, depth - 1);
        }

        return Color::zero();
    }

    let unit_direction = r.direction().to_unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn render_world_ch9(name: &str) {
    // World
    let mut world = HittableList {
        objects: Vec::new(),
    };

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.7, 0.3, 0.3));
    let material_left = Metal::new(Color::new(0.8, 0.8, 0.8), 0.3);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    world.add(Sphere {
        center: Point3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Box::new(material_ground),
    });
    world.add(Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Box::new(material_center),
    });
    world.add(Sphere {
        center: Point3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Box::new(material_left),
    });
    world.add(Sphere {
        center: Point3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Box::new(material_right),
    });

    // Camera
    let cam = Camera::new(RATIO);

    // create image buffer
    let mut imgbuf = ImageBuffer::new(WIDTH, HEIGHT);

    // Iterate over the coordinates and pixels of the image
    let len = WIDTH as u64 * HEIGHT as u64;
    let bar = ProgressBar::new(len);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        bar.inc(1);

        let mut c = Color::zero();
        for _s in 0..SAMPLES_PER_PIXEL {
            let u = (x as f64 + random_double()) / (WIDTH as f64 - 1f64);
            let v = ((HEIGHT - y) as f64 + random_double()) / (HEIGHT as f64 - 1f64);

            let r = cam.get_ray(u, v);

            c += ray_color_ch9(&r, &world, MAX_DEPTH);
        }

        *pixel = Rgb(c.to_u8_avg_gamma2(SAMPLES_PER_PIXEL));
    }
    bar.finish();

    // write the generated image (format is deduced based on extension)
    imgbuf.save(name).unwrap();
}

fn main() {
    render_world_ch9("out-ch9.png");

    // TODO 10 https://raytracing.github.io/books/RayTracingInOneWeekend.html#dielectrics
}
