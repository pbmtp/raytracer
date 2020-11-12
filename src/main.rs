#![feature(clamp)]
#![allow(dead_code)]

// https://raytracing.github.io/books/RayTracingInOneWeekend.html
// https://raytracing.github.io/books/RayTracingTheNextWeek.html
// TODO https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html

extern crate image;
extern crate indicatif;
extern crate rand;
extern crate rayon;

use indicatif::{HumanDuration, ProgressBar};
use rayon::prelude::*;

use std::time::Instant;

mod aabb;
mod aarect;
mod bvh;
mod camera;
mod cube;
mod hittable;
mod materials;
mod medium;
mod moving_sphere;
mod perlin;
mod ray;
mod rotate;
mod scene;
mod sphere;
mod texture;
mod tools;
mod translate;
mod vec3;

use hittable::Hittable;
use ray::Ray;
use scene::{Scene, SceneKind};
use tools::random_double;
use vec3::Color;

const BYTES_PER_PIXEL: usize = 3;

fn ray_color<T: Hittable>(r: &Ray, background: &Color, world: &T, depth: u32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return Color::zero();
    }

    if let Some(hr) = world.hit(&r, 0.001, std::f64::INFINITY) {
        let emitted = hr.material.emitted(hr.get_u(), hr.get_v(), &hr.get_p());

        let scatter = hr.material.scatter(&r, &hr);
        if let Some(bounce) = scatter.scattered {
            return emitted
                + scatter.attenuation * ray_color(&bounce, background, world, depth - 1);
        }

        return emitted;
    }

    *background
}

fn render(scene: &Scene, name: &str) {
    // Iterate over the coordinates and pixels of the image
    let width = scene.cfg.width;
    let height = scene.cfg.height;
    let samples_per_pixel = scene.cfg.samples_per_pixel;
    let max_depth = scene.cfg.max_depth;

    let len = width * height * BYTES_PER_PIXEL;
    // https://github.com/rust-lang/rust/issues/54628
    let mut pixels = vec![0u8; len];

    // FIXME https://docs.rs/indicatif/0.15.0/indicatif/#iterators
    let bar = ProgressBar::new(width as u64 * height as u64);
    pixels
        .par_chunks_mut(BYTES_PER_PIXEL)
        .into_par_iter()
        .enumerate()
        .for_each(|(idx, pixel)| {
            let y = height - (idx / width);
            let x = idx % width;

            let mut c = Color::zero();
            for _s in 0..samples_per_pixel {
                let u = (x as f64 + random_double()) / (width as f64 - 1f64);
                let v = (y as f64 + random_double()) / (height as f64 - 1f64);

                let r = scene.camera.get_ray(u, v);

                c += ray_color(&r, &scene.background, &scene.world, max_depth);
            }

            let avg = c.to_u8_avg_gamma2(samples_per_pixel);
            pixel[0] = avg[0];
            pixel[1] = avg[1];
            pixel[2] = avg[2];

            bar.inc(1);
        });
    bar.finish();

    // write the generated image (format is deduced based on extension)
    image::save_buffer(
        name,
        pixels.as_slice(),
        width as u32,
        height as u32,
        image::ColorType::Rgb8,
    )
    .unwrap();
}

fn main() {
    let scene = Scene::new(false, SceneKind::FinalScene, "data/1k/earth.jpg");

    let start = Instant::now();
    render(&scene, "out-test.png");
    println!(
        "Time elapsed rendering  scene is: {}",
        HumanDuration(start.elapsed())
    );
}
