use indicatif::{ProgressBar, ProgressStyle};

use crate::camera::ray::Ray;
use crate::hittable::Hittable;
use crate::scene::Scene;
use crate::tools::random_double_range;
use crate::vec3::{Color, Point3};

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

pub(crate) fn ray_color<T: Hittable>(r: &Ray, background: &Color, world: &T, depth: u32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return Color::zero();
    }

    if let Some(hr) = world.hit(r, 0.001, std::f64::INFINITY) {
        let emitted = hr.material.emitted(hr.get_u(), hr.get_v(), &hr.get_p());

        let scatter = hr.material.scatter(r, &hr);
        if let Some(bounce) = scatter.scattered {
            // 6.1 Original
            // return emitted
            //     + scatter.attenuation
            //         * hr.material.scattering_pdf(r, &hr, &bounce)
            //         * ray_color(&bounce, background, world, depth - 1) / scatter.pdf;

            // 9.2 Light Sampling (Hack)
            let on_light = Point3::new(
                random_double_range(213.0, 343.0),
                554.0,
                random_double_range(227.0, 332.0),
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
                    * ray_color(&scattered, background, world, depth - 1)
                    / pdf;
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
        RendererKind::ParallelCrossbeam => parallel_crossbeam::render(scene, &bar, &mut pixels),
        RendererKind::ParallelRayon => parallel_rayon::render(scene, &bar, &mut pixels),
        RendererKind::Sequential => sequential::render(scene, &bar, &mut pixels),
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
