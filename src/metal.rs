use glam::Vec3A;

use crate::{
    hit::HitRecord,
    material::Material,
    ray::{random_in_unit_sphere, Color, Ray},
};

pub(crate) fn reflect(v: Vec3A, n: Vec3A) -> Vec3A {
    v - 2.0 * v.dot(n) * n
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(ray_in.direction.normalize(), hit_record.normal);

        *scattered = Ray {
            origin: hit_record.p,
            direction: reflected + self.fuzz * random_in_unit_sphere(),
            time: ray_in.time,
        };
        *attenuation = self.albedo;

        return scattered.direction.dot(hit_record.normal) > 0.0;
    }
}
