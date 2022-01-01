#![allow(dead_code)]

// https://raytracing.github.io/books/RayTracingInOneWeekend.html
// https://raytracing.github.io/books/RayTracingTheNextWeek.html
// https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html

use clap::Parser;
use indicatif::HumanDuration;

use std::time::Instant;

mod camera;
mod geometry;
mod hittable;
mod materials;
mod onb;
mod pdf;
mod renderer;
mod scene;
mod texture;
mod tools;
mod vec3;

use renderer::{render, RendererKind};
use scene::{Scene, SceneKind};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Name of the output file
    #[clap(short, long, help_heading = "CONFIG", default_value = "out-test.png")]
    output: String,

    /// Name of the renderer to use
    #[clap(short, long, arg_enum, help_heading = "CONFIG", default_value = "parallel-crossbeam")]
    renderer: RendererKind,

    /// Name of the scene
    #[clap(short, long, arg_enum, help_heading = "CONFIG", default_value = "cornell-box-glass-sphere")]
    scene: SceneKind,

    /// Make objects move
    #[clap(short, long)]
    moving: bool,

    /// Image width
    #[clap(short = 'W', long, help_heading = "OVERRIDE")]
    width: Option<usize>,

    /// Image height
    #[clap(short = 'H', long, help_heading = "OVERRIDE")]
    height: Option<usize>,

    /// Number of samples per pixel
    #[clap(short = 'S', long, help_heading = "OVERRIDE")]
    samples_per_pixel: Option<u32>,
}

fn main() {
    let args = Args::parse();

    let scene = Scene::new(
        args.moving,
        args.scene,
        "data/1k/earth.jpg",
        args.width,
        args.height,
        args.samples_per_pixel,
    );

    let start = Instant::now();
    render(&scene, args.renderer, &args.output);
    println!(
        "Time elapsed rendering  scene is: {}",
        HumanDuration(start.elapsed())
    );
}
