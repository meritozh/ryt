use glam::DVec3;

use crate::{
    hit::HitRecord,
    material::Material,
    ray::{random_in_unit_sphere, Color, Ray},
};

fn reflect(v: DVec3, n: DVec3) -> DVec3 {
    v - 2.0 * v.dot(n) * n
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
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
        };
        *attenuation = self.albedo;

        return scattered.direction.dot(hit_record.normal) > 0.0;
    }
}
