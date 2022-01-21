use std::rc::Rc;

use glam::DVec3;
use image::ImageBuffer;
use rand::{distributions::Uniform, prelude::Distribution};
use ryt::{
    camera::Camera,
    dielectric::Dielectric,
    hit_list::HittableList,
    lambertian::Lambertian,
    material::Material,
    metal::Metal,
    ray::{ray_color, Color, Point3, Random},
    sphere::Sphere,
};

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    });
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let mut rng = rand::thread_rng();
    let choas = Uniform::new(0.0, 1.0);

    (-11..11).for_each(|a| {
        (-11..11).for_each(|b| {
            let choose_material = choas.sample(&mut rng);
            let center = Point3::new(
                a as f64 + 0.9 * choas.sample(&mut rng),
                0.2,
                b as f64 + choas.sample(&mut rng),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material> = if choose_material < 0.8 {
                    let albedo = Color::random() * Color::random();
                    Rc::new(Lambertian { albedo })
                } else if choose_material < 0.95 {
                    let albedo = Color::random_by(0.5, 1.0);
                    let fuzz = 0.0;
                    Rc::new(Metal { albedo, fuzz })
                } else {
                    Rc::new(Dielectric { ir: 1.5 })
                };
                world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
            }
        });
    });

    let material1 = Rc::new(Dielectric { ir: 1.5 });
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    });
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    return world;
}

fn main() {
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height: u32 = ((image_width as f64 / aspect_ratio).floor()) as u32;

    let samples_per_pixel = 500.0;
    let scale = 1.0 / samples_per_pixel;

    let max_depth = 50;

    let world = random_scene();

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = DVec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let apertune = 0.1;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        apertune,
        dist_to_focus,
    );

    let mut image = ImageBuffer::new(image_width, image_height);

    let mut rng = rand::thread_rng();
    let chaos = Uniform::new(0.0, 1.0);

    image.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let mut color = Color::new(0.0, 0.0, 0.0);
        (0..samples_per_pixel as i64).for_each(|_| {
            let u = (x as f64 + chaos.sample(&mut rng)) / (image_width - 1) as f64;
            let v = 1.0 - ((y as f64 - chaos.sample(&mut rng)) / (image_height - 1) as f64);
            let ray = camera.get_ray(u, v);

            color += ray_color(&ray, &world, max_depth);

            let color = color
                .to_array()
                .map(|val| val * scale)
                // gamma correction
                .map(|val| val.sqrt())
                .map(|val| (256.0 * val.clamp(0.0, 0.999)) as u8);
            *pixel = image::Rgb(color);
        })
    });

    image.save("output.png").unwrap();
}
