mod vec3;
mod ray;

use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use vec3::Vec3;
use ray::Ray;

fn color(r: &Ray) -> Vec3 {
    let unit_direction = r.direction().unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
}

fn main() {
    let nx = 600;
    let ny = 300;
    let file_name = "test.ppm";
    let write_file = File::create(file_name).unwrap();
    let mut writer = BufWriter::new(&write_file);
    writeln!(&mut writer, "P3\n{} {}\n255", nx, ny).expect(&format!("Error writing {}", file_name));

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..ny - 1).rev() {
        for i in 0..nx {
            let u: f64 = i as f64 / nx as f64;
            let v: f64 = j as f64 / ny as f64;

            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let col = color(&r);

            let ir: i32 = (255.99 * col.x) as i32;
            let ig: i32 = (255.99 * col.y) as i32;
            let ib: i32 = (255.99 * col.z) as i32;
            writeln!(&mut writer, "{} {} {}", ir, ig, ib).expect(&format!("Error writing {}", file_name));
        }
    }
}
