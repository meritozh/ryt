use glam::DVec3;

pub type Point3 = DVec3;
pub type Color = DVec3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: DVec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}

pub fn ray_color(ray: Ray) -> Color {
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
