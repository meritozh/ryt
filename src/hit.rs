use std::sync::Arc;

use glam::Vec3A;

use crate::{
    material::Material,
    ray::{Point3, Ray},
};

#[derive(Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3A,
    pub t: f32,
    pub front_face: bool,
    pub material: Option<Arc<dyn Material + Sync + Send>>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3A) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool;
}
