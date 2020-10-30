#![feature(clamp)]
#![allow(dead_code)]

// https://raytracing.github.io/books/RayTracingInOneWeekend.html

extern crate image;
extern crate indicatif;
extern crate rand;
extern crate rayon;

use image::{ImageBuffer, Rgb};
use indicatif::{HumanDuration, ProgressBar};
use rayon::prelude::*;

// use std::f64::consts::PI;
use std::time::Instant;

mod camera;
mod config;
mod hittable;
mod hittable_list;
mod materials;
mod ray;
mod sphere;
mod tools;
mod vec3;

use camera::Camera;
use config::Config;
use hittable::Hittable;
use hittable_list::HittableList;
use materials::{Dielectric, Lambertian, Metal};
use ray::Ray;
use sphere::Sphere;
use tools::{random_double, random_double_range};
use vec3::{Color, Point3, Vec3};

fn ray_color<T: Hittable>(r: &Ray, world: &T, depth: u32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return Color::zero();
    }

    if let Some(hr) = world.hit(&r, 0.001, std::f64::INFINITY) {
        let scatter = hr.material.scatter(&r, &hr);
        if let Some(r) = scatter.scattered {
            return scatter.attenuation * ray_color(&r, world, depth - 1);
        }

        return Color::zero();
    }

    let unit_direction = r.direction().to_unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn simple_scene() -> HittableList<Sphere> {
    // World
    let mut world = HittableList {
        objects: Vec::new(),
    };

    // ground
    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    world.add(Sphere {
        center: Point3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Box::new(material_ground),
    });

    // fixed part
    let material_center = Lambertian::new(Color::new(0.7, 0.3, 0.3));
    let material_left = Metal::new(Color::new(0.8, 0.8, 0.8), 0.3);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    world.add(Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 0.5,
        material: Box::new(material_center),
    });

    world.add(Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Box::new(material_left),
    });

    world.add(Sphere {
        center: Point3::new(0.0, 0.0, 1.0),
        radius: 0.5,
        material: Box::new(material_right),
    });

    world
}

fn random_scene() -> HittableList<Sphere> {
    // World
    let mut world = HittableList {
        objects: Vec::new(),
    };

    // ground
    let material_ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Sphere {
        center: Point3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Box::new(material_ground),
    });

    // random part
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                // shared_ptr<material> sphere_material;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Lambertian::new(albedo);
                    world.add(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(sphere_material),
                    });
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.add(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(sphere_material),
                    });
                } else {
                    // glass
                    let sphere_material = Dielectric::new(1.5);
                    world.add(Sphere {
                        center,
                        radius: 0.2,
                        material: Box::new(sphere_material),
                    });
                }
            }
        }
    }

    // fixed part
    let material1 = Dielectric::new(1.5);
    world.add(Sphere {
        center: Point3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(material1),
    });

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere {
        center: Point3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(material2),
    });

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere {
        center: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(material3),
    });

    world
}

fn render_world(cfg: &Config, name: &str) {
    // World
    let world = if cfg.quality {
        random_scene()
    } else {
        simple_scene()
    };

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        cfg.ratio,
        aperture,
        dist_to_focus,
    );

    // create image buffer
    let mut imgbuf = ImageBuffer::new(cfg.width, cfg.height);

    // Iterate over the coordinates and pixels of the image
    let len = cfg.width as u64 * cfg.height as u64;
    let bar = ProgressBar::new(len);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        bar.inc(1);

        let c: Color = (0..cfg.samples_per_pixel)
            .into_par_iter()
            .map(|_| {
                let u = (x as f64 + random_double()) / (cfg.width as f64 - 1f64);
                let v = ((cfg.height - y) as f64 + random_double()) / (cfg.height as f64 - 1f64);

                let r = cam.get_ray(u, v);

                ray_color(&r, &world, cfg.max_depth)
            })
            .collect::<Vec<Color>>()
            .iter()
            .sum();

        *pixel = Rgb(c.to_u8_avg_gamma2(cfg.samples_per_pixel));
    }
    bar.finish();

    // write the generated image (format is deduced based on extension)
    imgbuf.save(name).unwrap();
}

fn main() {
    let cfg = Config::speed();
    // let cfg = Config::quality();

    let start = Instant::now();
    render_world(&cfg, "out-test.png");
    println!(
        "Time elapsed rendering  scene is: {}",
        HumanDuration(start.elapsed())
    );
}
