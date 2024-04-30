mod vec3;
mod ray;
mod hit;
mod sphere;
mod camera;

use std::io::{self, Write, Result};
use rand::prelude::*;
use vec3::Vec3;
use ray::{Ray, Point3};
use hit::{Hit, World};
use sphere::Sphere;
use camera::Camera;


pub type Color = Vec3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u64 = 256;
const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
const SAMPLES_PER_PIXEL: u64 = 100;

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

fn ray_color(r: &Ray, world: &World) -> Color {
    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0))
    } else {
        let unit_direciton = r.direction().normalized();
        let t = 0.5 * (unit_direciton.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() -> Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let mut world = World::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let cam = Camera::new();
    
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);



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
                pixel_color += ray_color(&r, &world)
            }
            println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL));
        }
    }
    eprintln!("Done.");

    Ok(())
}
