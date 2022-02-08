use glam::Vec3A;
use rand::{distributions::Uniform, prelude::Distribution};

use crate::{
    hit::HitRecord,
    material::Material,
    metal::reflect,
    ray::{Color, Ray},
};

fn refract(uv: Vec3A, n: Vec3A, etai_over_etat: f32) -> Vec3A {
    let cos_theta = -uv.dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
    return r_out_perp + r_out_parallel;
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub struct Dielectric {
    pub ir: f32,
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);

        let refraction_ratio = if hit_record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = ray_in.direction.normalize();
        let cos_theta = (-unit_direction).dot(hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let mut rng = rand::thread_rng();
        let chaos = Uniform::new(0.0, 1.0);

        let direction = if cannot_refract
            || reflectance(cos_theta, refraction_ratio) > chaos.sample(&mut rng)
        {
            reflect(unit_direction, hit_record.normal)
        } else {
            refract(unit_direction, hit_record.normal, refraction_ratio)
        };

        *scattered = Ray {
            origin: hit_record.p,
            direction,
            time: ray_in.time,
        };
        return true;
    }
}
