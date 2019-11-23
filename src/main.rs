use rand;
use png;

mod vec3;
mod ray;
mod hitable;
mod camera;
mod renderer;

use png::HasParameters;
use std::io::BufWriter;
use std::path::Path;
use std::fs::File;
use std::f32;
use vec3::Vec3;
use hitable::*;
use camera::Camera;
use renderer::render;

fn main() {
    let nx = 200;
    let ny = 200;
    let ns = 100;

    let lookfrom = Vec3::new(10.0, 1.8, 8.0);
    let lookat = Vec3::new(0.0, 0.0, 0.5);
    let dist_to_focus = (lookfrom - Vec3::new(4.0, 1.0, 0.0)).length();

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 0.5, 0.0),
        35.0,
        (nx as f32) / (ny as f32),
        0.1,
        dist_to_focus,
    );

    let world = random_scene();
    let pixels = render(&world, &camera, nx, ny, ns);

    // Save image
    let path = Path::new(r"render.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, nx as u32, ny as u32);
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&pixels).unwrap();
}

fn random_scene() -> Vec<Box<dyn Hitable>> {
    let scene_c = Vec3::new(4.0, 0.0, 2.0);
    let mut spheres = vec![
        Sphere {
            center: Vec3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: Box::new(Lambertian {
                albedo: Vec3::new(0.7, 0.26, 0.10),
            }),
        },
        Sphere {
            center: Vec3::new(0.0, 1.0, 0.0),
            radius: 1.0,
            material: Box::new(Dielectric { ref_idx: 1.5 }),
        },
        Sphere {
            center: Vec3::new(-4.0, 1.0, 0.0),
            radius: 1.0,
            material: Box::new(Lambertian {
                albedo: Vec3::new(0.4, 0.2, 0.1),
            }),
        },
        Sphere {
            center: Vec3::new(4.0, 1.0, 0.0),
            radius: 1.0,
            material: Box::new(Metal {
                albedo: Vec3::new(0.7, 0.6, 0.5),
                fuzz: 0.0,
            }),
        },
    ];
    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(
                (a as f32) + 0.9 * rand::random::<f32>(),
                0.2,
                (b as f32) + 0.9 * rand::random::<f32>(),
            );

            if (center - scene_c).length() > 0.9 {
                let choose_mat = rand::random::<f32>();
                let material: Box<dyn Material>;

                if choose_mat < 0.8 {
                    material = Box::new(Lambertian {
                        albedo: Vec3::new(
                            rand::random::<f32>() * rand::random::<f32>(),
                            rand::random::<f32>() * rand::random::<f32>(),
                            rand::random::<f32>() * rand::random::<f32>(),
                        ),
                    });
                } else if choose_mat < 0.95 {
                    material = Box::new(Metal {
                        albedo: Vec3::new(
                            0.5 * (1.0 + rand::random::<f32>()),
                            0.5 * (1.0 + rand::random::<f32>()),
                            0.5 * (1.0 + rand::random::<f32>()),
                        ),
                        fuzz: 0.5 * rand::random::<f32>(),
                    })
                } else {
                    material = Box::new(Dielectric { ref_idx: 1.5 })
                }

                spheres.push(Sphere {
                    center,
                    radius: 0.2,
                    material,
                });
            }
        }
    }
    let world: Vec<Box<dyn Hitable>> = spheres
        .into_iter()
        .map(|s| Box::new(s) as Box<dyn Hitable>)
        .collect();
    world
}
