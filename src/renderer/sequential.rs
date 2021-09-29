use indicatif::ProgressBar;

use crate::scene::Scene;
use crate::tools::random_double;
use crate::vec3::Color;

use super::{ray_color, BYTES_PER_PIXEL};

pub(crate) fn render(scene: &Scene, bar: &ProgressBar, pixels: &mut Vec<u8>) {
    // Iterate over the coordinates and pixels of the image
    let width = scene.cfg.width;
    let height = scene.cfg.height;
    let samples_per_pixel = scene.cfg.samples_per_pixel;
    let max_depth = scene.cfg.max_depth;

    pixels
        .chunks_mut(BYTES_PER_PIXEL)
        .enumerate()
        .for_each(|(idx, pixel)| {
            let y = height - (idx / width);
            let x = idx % width;

            let mut c = Color::zero();
            for _s in 0..samples_per_pixel {
                let u = (x as f64 + random_double()) / (width as f64 - 1f64);
                let v = (y as f64 + random_double()) / (height as f64 - 1f64);

                let r = scene.camera.get_ray(u, v);

                c += ray_color(&r, &scene.background, &scene.world, &scene.light, max_depth);
            }

            let avg = c.to_u8_avg_gamma2(samples_per_pixel);
            pixel[0] = avg[0];
            pixel[1] = avg[1];
            pixel[2] = avg[2];

            bar.inc(1);
        });
}
