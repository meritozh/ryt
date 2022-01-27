use glam::Vec3A;
use rand::{distributions::Uniform, prelude::Distribution};

use crate::hit::{HitRecord, Hittable};

pub type Point3 = Vec3A;
pub type Color = Vec3A;

pub trait Random {
    fn random() -> Self;

    fn random_by(min: f32, max: f32) -> Self;
}

impl Random for Vec3A {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        let chaos = Uniform::new(0.0, 1.0);

        let x = chaos.sample(&mut rng);
        let y = chaos.sample(&mut rng);
        let z = chaos.sample(&mut rng);

        Self::new(x, y, z)
    }

    fn random_by(min: f32, max: f32) -> Self {
        let mut rng = rand::thread_rng();
        let chaos = Uniform::new(min, max);

        let x = chaos.sample(&mut rng);
        let y = chaos.sample(&mut rng);
        let z = chaos.sample(&mut rng);

        // println!("{x}, {y}, {z}");

        Self::new(x, y, z)
    }
}

pub(crate) trait NearZero {
    fn near_zero(&self) -> bool;
}

impl NearZero for Vec3A {
    fn near_zero(&self) -> bool {
        let precision = 1e-8;
        self.to_array().iter().all(|val| f32::abs(*val) < precision)
    }
}

#[derive(Debug, Default)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3A,
}

impl Ray {
    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }
}

pub fn hit_sphere(center: Point3, radius: f32, ray: &Ray) -> f32 {
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

#[inline]
pub fn random_in_unit_sphere() -> Vec3A {
    loop {
        let p = Vec3A::random_by(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            break p;
        }
    }
}

#[inline]
pub fn random_unit_vector() -> Vec3A {
    random_in_unit_sphere().normalize()
}

#[allow(dead_code)]
#[inline]
fn random_in_hemisphere(normal: &Vec3A) -> Vec3A {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(*normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

#[inline]
pub fn random_in_unit_disk() -> Vec3A {
    let mut rng = rand::thread_rng();
    let choas = Uniform::new(-1.0, 1.0);

    loop {
        let p = Vec3A::new(choas.sample(&mut rng), choas.sample(&mut rng), 0.0);
        if p.length_squared() < 1.0 {
            break p;
        }
    }
}

pub fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    let mut record = HitRecord::default();

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(ray, 0.001, f32::INFINITY, &mut record) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();

        if record.material.as_ref().map_or(false, |v| {
            v.scatter(ray, &record, &mut attenuation, &mut scattered)
        }) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}
