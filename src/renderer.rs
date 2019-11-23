use indicatif::HumanDuration;
use rand;

use std::f32;
use std::time::Instant;

use crate::camera::Camera;
use crate::hitable::*;
use crate::ray::Ray;
use crate::vec3;
use crate::vec3::Vec3;

pub fn render(world: &dyn Hitable, camera: &Camera, nx: usize, ny: usize, ns: usize) -> Vec<u8> {
    let bar = indicatif::ProgressBar::new(ny as u64);
    let started = Instant::now();
    bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:35.cyan/blue}] [{percent}%]")
            .progress_chars("=> "),
    );
    let mut pixels: Vec<u8> = Vec::with_capacity(nx * ny * 3);
    println!("Generating image...");
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..ns {
                let u: f32 = (i as f32 + rand::random::<f32>()) / nx as f32;
                let v: f32 = (j as f32 + rand::random::<f32>()) / ny as f32;

                let r = camera.get_ray(u, v);
                col += color(r, world, 0);
            }

            col /= ns as f32;
            col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());

            pixels.push((255.99 * col.x) as u8);
            pixels.push((255.99 * col.y) as u8);
            pixels.push((255.99 * col.z) as u8);
        }
        bar.inc(1);
    }
    bar.finish();
    println!("Image generated in {}", HumanDuration(started.elapsed()));
    pixels
}

fn color(r: Ray, world: &dyn Hitable, depth: i32) -> Vec3 {
    let hit = world.hit(&r, 0.001, f32::MAX);
    match hit {
        Some(rec) => {
            if depth < 50 {
                match rec.material.scatter(&r, &rec) {
                    Some(scatter) => {
                        if let Some(bounce) = scatter.ray {
                            return scatter.color * color(bounce, world, depth + 1);
                        }
                    }
                    None => {}
                }
            }
            return Vec3::new(0.0, 0.0, 0.0);
        }
        None => {
            let unit_direction = vec3::unit_vector(r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
        }
    }
}
