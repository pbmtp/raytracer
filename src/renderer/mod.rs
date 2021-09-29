use indicatif::{ProgressBar, ProgressStyle};

use crate::camera::ray::Ray;
use crate::hittable::Hittable;
use crate::pdf::cosine::CosinePdf;
use crate::pdf::hittable::HittablePdf;
use crate::pdf::mixture::MixturePdf;
use crate::pdf::Pdf;
use crate::scene::Scene;
// use crate::tools::random_double_range;
use crate::vec3::Color;

pub mod parallel_crossbeam;
pub mod parallel_rayon;
pub mod sequential;

// TODO render to frame buffer using https://lib.rs/crates/pixels or https://lib.rs/crates/minifb

pub(crate) const BYTES_PER_PIXEL: usize = 3;

pub enum RendererKind {
    ParallelCrossbeam,
    ParallelRayon,
    Sequential,
}

pub(crate) fn ray_color(
    r: &Ray,
    background: &Color,
    world: &Vec<Box<dyn Hittable>>,
    light: &Vec<Box<dyn Hittable>>,
    depth: u32,
) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return Color::zero();
    }

    if let Some(hr) = world.hit(r, 0.001, std::f64::INFINITY) {
        let emitted = hr
            .material
            .emitted(r, &hr, hr.get_u(), hr.get_v(), &hr.get_p());

        let scatter = hr.material.scatter(r, &hr);
        if let Some(_scattered) = scatter.scattered {
            /*
            // 6.1 Original
            return emitted
                + scatter.attenuation
                   * hr.material.scattering_pdf(r, &hr, &bounce)
                   * ray_color(&scattered, background, world, depth - 1) / scatter.pdf;
            */

            /*
            // 9.2 Light Sampling (Hack)
            let on_light = crate::vec3::Point3::new(
                crate::tools::random_double_range(213.0, 343.0),
                554.0,
                crate::tools::random_double_range(227.0, 332.0),
            );
            let to_light = on_light - hr.get_p();
            let distance_squared = to_light.length_squared();
            let to_light = to_light.to_unit_vector();

            if to_light.dot(hr.get_normal()) < 0.0 {
                return emitted;
            }

            let light_area = (343.0 - 213.0) * (332.0 - 227.0);
            let light_cosine = to_light.y().abs();
            if light_cosine < 0.000001 {
                return emitted;
            }

            let pdf = distance_squared / (light_cosine * light_area);
            let scattered = Ray::new(hr.get_p(), to_light, r.time());
            return emitted
                + scatter.attenuation
                    * hr.material.scattering_pdf(r, &hr, &scattered)
                    * ray_color(&scattered, background, world, light, depth - 1)
                    / pdf;
            */

            /*
            // 10.1 An Average of Lighting and Reflection
            let p = CosinePdf::new(&hr.get_normal());
            let scattered = Ray::new(hr.get_p(), p.generate(), r.time());
            let pdf_val = p.value(&scattered.direction());

            return emitted
                + scatter.attenuation
                    * hr.material.scattering_pdf(r, &hr, &scattered)
                    * ray_color(&scattered, background, world, light, depth - 1)
                    / pdf_val;
            */

            /*
            // 10.2 Sampling Directions towards a Hittable
            let light_pdf = HittablePdf::new(light, hr.get_p());
            let scattered = Ray::new(hr.get_p(), light_pdf.generate(), r.time());
            let pdf_val = light_pdf.value(&scattered.direction());

            return emitted
                + scatter.attenuation
                    * hr.material.scattering_pdf(r, &hr, &scattered)
                    * ray_color(&scattered, background, world, light, depth - 1)
                    / pdf_val;
            */

            // 10.3 Mixture Pdf
            let p0 = HittablePdf::new(light, hr.get_p());
            let p1 = CosinePdf::new(&hr.get_normal());

            let mixed_pdf = MixturePdf::new(p0, p1);

            let scattered = Ray::new(hr.get_p(), mixed_pdf.generate(), r.time());
            let pdf_val = mixed_pdf.value(&scattered.direction());

            return emitted
                + scatter.attenuation
                    * hr.material.scattering_pdf(r, &hr, &scattered)
                    * ray_color(&scattered, background, world, light, depth - 1)
                    / pdf_val;
        }

        return emitted;
    }

    *background
}

pub fn render(scene: &Scene, renderer: RendererKind, name: &str) {
    let width = scene.cfg.width;
    let height = scene.cfg.height;

    let len = width * height * BYTES_PER_PIXEL;
    // https://github.com/rust-lang/rust/issues/54628
    let mut pixels = vec![0u8; len];

    // FIXME https://docs.rs/indicatif/0.15.0/indicatif/#iterators
    let bar_len = width as u64 * height as u64;
    let bar = ProgressBar::new(bar_len);
    bar.set_style(ProgressStyle::default_bar().template(
        "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] ({pos}/{len}, ETA {eta})",
    ));
    bar.set_draw_delta(bar_len / 100);

    match renderer {
        RendererKind::ParallelCrossbeam => parallel_crossbeam::render(&scene, &bar, &mut pixels),
        RendererKind::ParallelRayon => parallel_rayon::render(&scene, &bar, &mut pixels),
        RendererKind::Sequential => sequential::render(&scene, &bar, &mut pixels),
    }

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
