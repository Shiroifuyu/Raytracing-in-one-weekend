use rand;

use crate::vec3::*;
use crate::ray::Ray;
use std::f32;

#[derive(Debug)]
pub struct Camera {
  pub origin: Vec3,
  pub lower_left_corner: Vec3,
  pub horizontal: Vec3,
  pub vertical: Vec3,
  pub lens_radius: f32,
  u: Vec3,
  v: Vec3,
  w: Vec3
}

impl Camera {
  pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Camera {
    let theta = vfov * f32::consts::PI / 180.0;
    let half_height = f32::tan(theta / 2.0);
    let half_width = aspect * half_height;    

    let w = unit_vector(lookfrom - lookat);
    let u = unit_vector(vup.cross(w));
    let v = w.cross(u);

    Camera {
        lower_left_corner: lookfrom - u * half_width * focus_dist - v * half_height * focus_dist - w * focus_dist,
        horizontal: u * 2.0 * half_width * focus_dist,
        vertical: v * 2.0 * half_height * focus_dist,
        origin: lookfrom,
        lens_radius: aperture / 2.0,
        u,
        v,
        w
    }
  }

  pub fn get_ray(&self, s: f32, t: f32) -> Ray {
    let rd = random_in_unit_disk() * self.lens_radius;
    let offset = self.u * rd.x() + self.v * rd.y();

    Ray::new(self.origin + offset, self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset)
  }
}

fn random_in_unit_disk() -> Vec3 {
    loop {
      let p = Vec3::new(rand::random::<f32>() * 2.0, rand::random::<f32>(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
      if p.dot(p) < 1.0 {
        return p;
      }
    }
}