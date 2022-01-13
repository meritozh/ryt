use image::ImageBuffer;
use rand::{distributions::Uniform, prelude::Distribution};
use ryt::{
    camera::Camera,
    hit_list::HittableList,
    ray::{ray_color, Color, Point3},
    sphere::Sphere,
};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height: u32 = ((image_width as f64 / aspect_ratio).floor()) as u32;

    let samples_per_pixel = 100.0;
    let scale = 1.0 / samples_per_pixel;

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new();

    let mut image = ImageBuffer::new(image_width, image_height);

    let mut rng = rand::thread_rng();
    let chaos = Uniform::new(0.0, 1.0);

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let mut color = Color::new(0.0, 0.0, 0.0);
        (0..samples_per_pixel as i64).for_each(|_| {
            let u = (x as f64 + chaos.sample(&mut rng)) / (image_width - 1) as f64;
            let v = 1.0 - ((y as f64 + chaos.sample(&mut rng)) / (image_height - 1) as f64);
            let ray = camera.get_ray(u, v);

            color += ray_color(&ray, &world);

            let color = color
                .to_array()
                .map(|val| val * scale)
                .map(|val| (256.0 * val.clamp(0.0, 0.999)) as u8);

            *pixel = image::Rgb(color);
        });
    }

    image.save("output.png").unwrap();
}
