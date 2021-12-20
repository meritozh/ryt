use glam::DVec3;
use image::ImageBuffer;
use ryt::ray::{ray_color, Point3, Ray};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height: u32 = ((image_width as f64 / aspect_ratio).floor()) as u32;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = DVec3::new(viewport_width, 0.0, 0.0);
    let vertical = DVec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - DVec3::new(0.0, 0.0, focal_length);

    let mut image = ImageBuffer::new(image_width, image_height);

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let u = x as f64 / (image_width - 1) as f64;
        let v = 1.0 - (y as f64 / (image_height - 1) as f64);
        let r = Ray {
            origin,
            direction: lower_left_corner + u * horizontal + v * vertical - origin,
        };
        let color: [u8; 3] = ray_color(r)
            .to_array()
            .map(|val| (val * 255.999) as u8)
            .try_into()
            .unwrap();
        *pixel = image::Rgb(color);
    }

    image.save("output.png").unwrap();
}
