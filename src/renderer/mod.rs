use indicatif::{ProgressBar, ProgressStyle};

use crate::camera::ray::Ray;
use crate::hittable::Hittable;
use crate::scene::Scene;
use crate::vec3::Color;

pub mod parallel_rayon;
pub mod sequential;

// TODO render to frame buffer using https://lib.rs/crates/pixels or https://lib.rs/crates/minifb

pub(crate) const BYTES_PER_PIXEL: usize = 3;

pub enum RendererKind {
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
            return emitted
                + scatter.attenuation * ray_color(&bounce, background, world, depth - 1);
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
