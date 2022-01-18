use glam::DVec3;
use rand::{distributions::Uniform, prelude::Distribution};

use crate::hit::{HitRecord, Hittable};

pub type Point3 = DVec3;
pub type Color = DVec3;

trait Random {
    fn random() -> Self;

    fn random_by(min: f64, max: f64) -> Self;
}

impl Random for DVec3 {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        let chaos = Uniform::new(0.0, 1.0);

        let x = chaos.sample(&mut rng);
        let y = chaos.sample(&mut rng);
        let z = chaos.sample(&mut rng);

        // println!("{x}, {y}, {z}");

        Self::new(x, y, z)
    }

    fn random_by(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        let chaos = Uniform::new(min, max);

        let x = chaos.sample(&mut rng);
        let y = chaos.sample(&mut rng);
        let z = chaos.sample(&mut rng);

        // println!("{x}, {y}, {z}");

        Self::new(x, y, z)
    }
}

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

fn random_in_unit_sphere() -> DVec3 {
    loop {
        let p = DVec3::random_by(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            break p;
        }
    }
}

pub fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i64) -> Color {
    let mut record = HitRecord::default();

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(ray, 0.001, f64::INFINITY, &mut record) {
        let target = record.p + record.normal + random_in_unit_sphere();
        return 0.5
            * ray_color(
                &Ray {
                    origin: record.p,
                    direction: target - record.p,
                },
                world,
                depth - 1,
            );
    }
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}
