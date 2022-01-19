use std::rc::Rc;

use crate::hit::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Point3;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<Box<dyn Material>>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Box<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material: Rc::new(material),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

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

        let outward_normal = (record.p - self.center) / self.radius;
        record.set_face_normal(ray, outward_normal);
        record.material = Some(self.material.clone());

        return true;
    }
}
