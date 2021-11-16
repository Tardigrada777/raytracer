use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f32, aspect: f32) -> Camera {
        let mut u = Vec3::default();
        let mut v = Vec3::default();
        let mut w = Vec3::default();
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let origin = look_from;
        w = Vec3::unit_vector(&(look_from - look_at));
        u = Vec3::unit_vector(&(Vec3::cross(&vup, &w)));
        v = Vec3::cross(&w, &u);
        Camera {
            origin,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            lower_left_corner: origin - half_width * u - half_height * v - w,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
