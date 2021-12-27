use crossbeam_channel::bounded;
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

    let workers_count = num_cpus::get();

    let (jobs_tx, jobs_rx) = bounded(workers_count);
    let (res_tx, res_rx) = bounded(workers_count);

    crossbeam::scope(move |s| {
        // produce data to workers
        s.spawn(move |_| {
            // TODO by blocks
            // TODO shuffle
            for y in 0..height {
                for x in 0..width {
                    jobs_tx.send((x, y)).unwrap();
                }
            }
            drop(jobs_tx); // this will stop workers
        });

        // workers
        for _ in 0..workers_count {
            let rx = jobs_rx.clone();
            let tx = res_tx.clone();
            s.spawn(move |_| {
                while let Ok(coords) = rx.recv() {
                    let x = coords.0;
                    let y = coords.1;

                    // render pixel
                    let mut c = Color::zero();
                    for _s in 0..samples_per_pixel {
                        let u = (x as f64 + random_double()) / (width as f64 - 1f64);
                        let v = ((height - y) as f64 + random_double()) / (height as f64 - 1f64);

                        let r = scene.camera.get_ray(u, v);

                        c +=
                            ray_color(&r, &scene.background, &scene.world, &scene.light, max_depth);
                    }

                    let avg = c.to_u8_avg_gamma2(samples_per_pixel);
                    tx.send((x, y, avg[0], avg[1], avg[2])).unwrap();

                    bar.inc(1);
                }
            });
        }
        drop(jobs_rx);
        drop(res_tx);

        // main thread consume results
        while let Ok(res) = res_rx.recv() {
            // TODO preview
            let (x, y, r, g, b) = res;
            let idx = (y * width + x) * BYTES_PER_PIXEL;
            pixels[idx] = r;
            pixels[idx + 1] = g;
            pixels[idx + 2] = b;
        }
    })
    .unwrap();
}
