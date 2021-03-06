use std::fmt::Display;
use std::str::FromStr;

use clap::{ArgEnum, PossibleValue};
use indicatif::{ProgressBar, ProgressStyle};

use crate::camera::ray::Ray;
use crate::hittable::Hittable;
use crate::materials::ScatterRecord;
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

#[derive(ArgEnum, Debug, Clone, Copy)]
pub enum RendererKind {
    ParallelCrossbeam,
    ParallelRayon,
    Sequential,
}

impl FromStr for RendererKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("Invalid variant: {}", s))
    }
}

impl Display for RendererKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl RendererKind {
    pub fn possible_values() -> impl Iterator<Item = PossibleValue<'static>> {
        Self::value_variants()
            .iter()
            .filter_map(ArgEnum::to_possible_value)
    }
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
        if let Some(srec) = scatter {
            match srec {
                ScatterRecord::Specular { attenuation, ray } => {
                    return attenuation * ray_color(&ray, background, world, light, depth - 1);
                }
                ScatterRecord::Diffuse { attenuation, pdf } => {
                    let light_pdf = HittablePdf::new(light, hr.get_p());
                    let mixed_pdf = MixturePdf::new(light_pdf, pdf);

                    let scattered = Ray::new(hr.get_p(), mixed_pdf.generate(), r.time());
                    let pdf_val = mixed_pdf.value(&scattered.direction());

                    return emitted
                        + attenuation
                            * hr.material.scattering_pdf(r, &hr, &scattered)
                            * ray_color(&scattered, background, world, light, depth - 1)
                            / pdf_val;
                }
            }
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
