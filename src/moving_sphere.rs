use std::sync::Arc;

use crate::{hit::Hittable, material::Material, ray::Point3};

pub struct MovingSphere {
    center0: Point3,
    center1: Point3,
    time0: f32,
    time1: f32,
    radius: f32,
    material: Arc<dyn Material + Sync + Send>,
}

impl MovingSphere {
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: f32,
        time1: f32,
        radius: f32,
        material: Arc<dyn Material + Sync + Send>,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f32) -> Point3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
        record: &mut crate::hit::HitRecord,
    ) -> bool {
        let oc = ray.origin - self.center(ray.time);
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        record.t = root;
        record.p = ray.at(record.t);

        let outward_normal = (record.p - self.center(ray.time)) / self.radius;
        record.set_face_normal(ray, outward_normal);
        record.material = Some(self.material.clone());

        return true;
    }
}
