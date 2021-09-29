#![allow(dead_code)]

// https://raytracing.github.io/books/RayTracingInOneWeekend.html
// https://raytracing.github.io/books/RayTracingTheNextWeek.html
// IN PROGRESS https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html

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

fn main() {
    let scene = Scene::new(false, SceneKind::CornellBox, "data/1k/earth.jpg");

    let start = Instant::now();
    render(&scene, RendererKind::ParallelCrossbeam, "out-test.png");
    println!(
        "Time elapsed rendering  scene is: {}",
        HumanDuration(start.elapsed())
    );
}
