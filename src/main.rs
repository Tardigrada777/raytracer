mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

// TODO: move this into lib.rs
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

// fn write_ppm(w: i32, h: i32, max_value: i32) {
//     println!("P3\n{} {}\n{}", w, h, max_value);
//     for j in (0..h).rev() {
//         for i in 0..w {
//             let r: f32 = i as f32 / w as f32;
//             let g: f32 = j as f32 / h as f32;
//             let b: f32 = 0.2;

//             let ir: i32 = (255.99 * r) as i32;
//             let ig: i32 = (255.99 * g) as i32;
//             let ib: i32 = (255.99 * b) as i32;

//             println!("{} {} {}", ir, ig, ib);
//         }
//     }
// }

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
        return Vec3::new(
            rec.normal().x() + 1.0,
            rec.normal().y() + 1.0,
            rec.normal().z() + 1.0,
        ) * 0.5;
    } else {
        let unit_direction = Vec3::unit_vector(&r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    let width: i32 = 800;
    let height: i32 = 400;
    let max_value: i32 = 255;
    // write_ppm(width, height, max_value);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    list.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    let world: HittableList = HittableList::new(list);
    println!("P3\n{} {}\n{}", width, height, max_value);

    for j in (0..height).rev() {
        for i in 0..width {
            let u: f32 = i as f32 / width as f32;
            let v: f32 = j as f32 / height as f32;
            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let p = r.point_at_parameter(2.0);
            let color = color(&r, &world);
            let ir: i32 = (255.99 * color.r()) as i32;
            let ig: i32 = (255.99 * color.g()) as i32;
            let ib: i32 = (255.99 * color.b()) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
