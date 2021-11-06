mod ray;
mod vec3;

use ray::Ray;
use vec3::Vec3;

fn write_ppm(w: i32, h: i32, max_value: i32) {
    println!("P3\n{} {}\n{}", w, h, max_value);
    for j in (0..h).rev() {
        for i in 0..w {
            let r: f32 = i as f32 / w as f32;
            let g: f32 = j as f32 / h as f32;
            let b: f32 = 0.2;

            let ir: i32 = (255.99 * r) as i32;
            let ig: i32 = (255.99 * g) as i32;
            let ib: i32 = (255.99 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn color(r: &Ray) -> Vec3 {
    let unit_direction = Vec3::unit_vector(&r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let width: i32 = 800;
    let height: i32 = 600;
    let max_value: i32 = 255;
    // write_ppm(width, height, max_value);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    println!("P3\n{} {}\n{}", width, height, max_value);
    for j in (0..height).rev() {
        for i in 0..width {
            let u: f32 = i as f32 / width as f32;
            let v: f32 = j as f32 / height as f32;
            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let color = color(&r);
            let ir: i32 = (255.99 * color.r()) as i32;
            let ig: i32 = (255.99 * color.g()) as i32;
            let ib: i32 = (255.99 * color.b()) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
