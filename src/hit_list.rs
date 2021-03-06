use std::sync::Arc;

use crate::{
    hit::{HitRecord, Hittable},
    ray::Ray,
};

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable + Sync + Send>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::default(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable + Sync + Send>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        self.objects.iter().for_each(|object| {
            if object.hit(ray, t_min, closest_so_far, record) {
                hit_anything = true;
                closest_so_far = record.t;
            }
        });

        hit_anything
    }
}
