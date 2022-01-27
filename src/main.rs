use std::sync::Arc;

use glam::Vec3A;
use image::RgbImage;
use indicatif::ProgressBar;
use rand::{distributions::Uniform, prelude::Distribution};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

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

    let ground_material = Arc::new(Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    });
    world.add(Arc::new(Sphere::new(
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
                a as f32 + 0.9 * choas.sample(&mut rng),
                0.2,
                b as f32 + choas.sample(&mut rng),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material + Send + Sync> = if choose_material < 0.8 {
                    let albedo = Color::random() * Color::random();
                    Arc::new(Lambertian { albedo })
                } else if choose_material < 0.95 {
                    let albedo = Color::random_by(0.5, 1.0);
                    let fuzz = 0.0;
                    Arc::new(Metal { albedo, fuzz })
                } else {
                    Arc::new(Dielectric { ir: 1.5 })
                };
                world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        });
    });

    let material1 = Arc::new(Dielectric { ir: 1.5 });
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    });
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    return world;
}

fn main() {
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height: u32 = ((image_width as f32 / aspect_ratio).floor()) as u32;

    let samples_per_pixel = 500.0;
    let scale = 1.0 / samples_per_pixel;

    let max_depth = 50;

    let world = random_scene();

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3A::new(0.0, 1.0, 0.0);
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

    let chaos = Uniform::new(0.0, 1.0);

    let len = image_width * image_height;
    let progress_bar = ProgressBar::new(len as u64);

    let pixels: Vec<u8> = (0..len)
        .into_par_iter()
        .map(|idx| {
            let x = idx as u32 % image_width;
            let y = idx as u32 / image_width;

            let mut rng = rand::thread_rng();
            let mut color = Color::new(0.0, 0.0, 0.0);

            (0..samples_per_pixel as i32).for_each(|_| {
                let u = (x as f32 + chaos.sample(&mut rng)) / (image_width - 1) as f32;
                let v = 1.0 - ((y as f32 - chaos.sample(&mut rng)) / (image_height - 1) as f32);
                let ray = camera.get_ray(u, v);

                color += ray_color(&ray, &world, max_depth);
            });

            progress_bar.inc(1);

            color
                .to_array()
                .map(|val| val * scale)
                // gamma correction
                .map(|val| val.sqrt())
                .map(|val| (256.0 * val.clamp(0.0, 0.999)) as u8)
        })
        .flat_map(|val| val)
        .collect();

    let image = RgbImage::from_raw(image_width, image_height, pixels).unwrap();
    image.save("output.png").unwrap();

    progress_bar.finish();
}
