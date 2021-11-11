use crate::hittable::HitRecord;
use crate::random_in_unit_sphere;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f32 },
    Dielectric {},
}

impl Default for Material {
    fn default() -> Self {
        Material::Lambertian {
            albedo: Vec3::default(),
        }
    }
}

pub fn scatter(
    material: &Material,
    ray_in: &Ray,
    rec: &HitRecord,
    attenuation: &mut Vec3,
    scattered: &mut Ray,
) -> bool {
    match material {
        &Material::Lambertian { albedo } => {
            let target = rec.p + rec.normal + random_in_unit_sphere();
            *scattered = Ray::new(rec.p, target - rec.p);
            *attenuation = albedo;
            return true;
        }
        &Material::Metal { albedo, fuzz } => {
            let mut f = 1.0;
            if fuzz < 1.0 {
                f = fuzz;
            }
            let reflected = reflect(&Vec3::unit_vector(&ray_in.direction()), &rec.normal);
            *scattered = Ray::new(rec.p, reflected + f * random_in_unit_sphere());
            *attenuation = albedo;
            return Vec3::dot(&scattered.direction(), &rec.normal) > 0.0;
        }
        &Material::Dielectric {} => false,
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * Vec3::dot(&v, &n) * *n
}
