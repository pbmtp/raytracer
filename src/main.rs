#![allow(dead_code)]

// https://raytracing.github.io/books/RayTracingInOneWeekend.html
// https://raytracing.github.io/books/RayTracingTheNextWeek.html
// IN PROGRESS https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html

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
    #[clap(short, long, default_value = "out-test.png")]
    output: String,

    /// Name of the renderer to use
    #[clap(short, long, arg_enum, default_value = "parallel-crossbeam")]
    renderer: RendererKind,

    /// Name of the scene
    #[clap(short, long, arg_enum, default_value = "cornell-box-glass-sphere")]
    scene: SceneKind,

    /// Make objects move
    #[clap(short, long)]
    moving: bool,
}

fn main() {
    let args = Args::parse();

    let scene = Scene::new(args.moving, args.scene, "data/1k/earth.jpg");

    let start = Instant::now();
    render(&scene, args.renderer, &args.output);
    println!(
        "Time elapsed rendering  scene is: {}",
        HumanDuration(start.elapsed())
    );
}
