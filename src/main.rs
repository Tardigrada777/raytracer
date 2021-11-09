use rand::prelude::*;

mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

// TODO: move this into lib.rs
use camera::Camera;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.origin() - center;
    let a = Vec3::dot(&ray.direction(), &ray.direction());
    let b = 2.0 * Vec3::dot(&oc, &ray.direction());
    let c = Vec3::dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / 2.0 * a;
    }
}

fn color(r: &Ray, world: &HittableList) -> Vec3 {
    let mut rec = HitRecord::default();
    if world.hit(&r, 0.0, std::f32::MAX, &mut rec) {
        let target = rec.p() + rec.normal() + random_in_unit_sphere();
        return 0.5 * color(&Ray::new(rec.p(), target - rec.p()), world);
    } else {
        let unit_direction = Vec3::unit_vector(&r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3::default();
    let mut rng = rand::thread_rng();
    loop {
        p = 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
            - Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

fn main() {
    let width = 800;
    let height = 400;
    let samples = 100;
    let max_value: i32 = 255;

    let camera = Camera::new();
    let mut rng = rand::thread_rng();

    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    list.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    let world: HittableList = HittableList::new(list);
    println!("P3\n{} {}\n{}", width, height, max_value);

    for j in (0..height).rev() {
        for i in 0..width {
            let mut col = Vec3::default();
            for _ in 0..samples {
                let u: f32 = (i as f32 + rng.gen::<f32>()) / width as f32;
                let v: f32 = (j as f32 + rng.gen::<f32>()) / height as f32;
                let r = &camera.get_ray(u, v);
                col = col + color(&r, &world);
            }
            col = col / samples as f32;
            col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());
            let ir: i32 = (255.99 * col.r()) as i32;
            let ig: i32 = (255.99 * col.g()) as i32;
            let ib: i32 = (255.99 * col.b()) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
