mod vec3;
mod ray;
mod hit;
mod sphere;
mod camera;
mod material;

use std::io::{self, Write, Result};
use std::rc::Rc;
use rand::prelude::*;
use vec3::Vec3;
use ray::{Ray, Point3};
use hit::{Hit, World, HitRecord};
use sphere::Sphere;
use camera::Camera;
use material::{Lambertian, Metal};

pub type Color = Vec3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u64 = 256;
const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
const SAMPLES_PER_PIXEL: u64 = 100;
const MAX_DEPTH: u64 = 5;

pub fn write_color(color: Vec3) {
    let r = (255.999 * color.x()) as i32;
    let g = (255.999 * color.y()) as i32;
    let b = (255.999 * color.z()) as i32;

    println!("{} {} {}", r, g, b);
}

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().length().powi(2);
    let half_b = oc.dot(r.direction());
    let c = oc.length().powi(2) - radius * radius;
    let discriminant = half_b * half_b - a * c;


    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn ray_color(r: &Ray, world: &World, depth: u64) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Color::new(1.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = r.direction().normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
    }
}

fn main() -> Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let mut world = World::new();

    let mat_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let mat_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let mat_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

    let sphere_ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, mat_ground);
    let sphere_center = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, mat_center);
    let sphere_left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, mat_left);
    let sphere_right = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, mat_right);

    world.push(Box::new(sphere_ground));
    world.push(Box::new(sphere_center));
    world.push(Box::new(sphere_left));
    world.push(Box::new(sphere_right));

    let cam = Camera::new();
    writeln!(handle, "P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    
    let mut rng = rand::thread_rng();

    for col in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {} ", IMAGE_HEIGHT - col);

        for row in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let random_u : f64 = rng.gen();
                let random_v: f64 = rng.gen();

                let u = ((row as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((col as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH)
            }
            println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL));
        }
    }

    eprintln!("Done.");

    Ok(())
}
