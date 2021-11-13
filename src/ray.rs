use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray { a, b }
    }

    pub fn origin(self) -> Vec3 {
        self.a
    }

    pub fn direction(self) -> Vec3 {
        self.b
    }

    pub fn point_at_parameter(self, t: f32) -> Vec3 {
        self.a + self.b * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_ray_origin() {
    //     todo!();
    // }

    // #[test]
    // fn test_ray_direction() {
    //     todo!();
    // }

    // #[test]
    // fn test_ray_point_at_parameter() {
    //     todo!();
    // }
}
