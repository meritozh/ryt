use glam::DVec3;

use crate::hit::{HitRecord, Hittable};

pub type Point3 = DVec3;
pub type Color = DVec3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: DVec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}

pub fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.direction.length_squared();
    let half_b = oc.dot(ray.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

pub fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    let mut record = HitRecord::default();
    if world.hit(ray, 0.0, f64::INFINITY, &mut record) {
        return 0.5 * (record.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
