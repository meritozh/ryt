use glam::Vec3A;
use rand::{distributions::Uniform, prelude::Distribution};

use crate::ray::{random_in_unit_disk, Point3, Ray};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3A,
    vertical: Vec3A,
    u: Vec3A,
    v: Vec3A,
    #[allow(dead_code)]
    w: Vec3A,
    lens_radius: f32,
    time0: f32,
    time1: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3A,
        vfov: f32,
        aspect_ratio: f32,
        apertune: f32,
        focus_dist: f32,
        time0: f32,
        time1: f32,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;
        let lens_radius = apertune / 2.0;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius,
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        let mut rng = rand::thread_rng();
        let chaos = Uniform::new(self.time0, self.time1);

        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
            time: chaos.sample(&mut rng),
        }
    }
}
