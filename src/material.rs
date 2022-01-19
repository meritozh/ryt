use crate::{
    hit::HitRecord,
    ray::{Color, Ray},
};

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}
