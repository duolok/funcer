use crate::vec3::Vec3;

pub type Point3 = Vec3;

#[derive(Debug)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Vec3 ) -> Self {
        Self {
            origin: *origin, 
            direction: *direction
        }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t_val: f64) -> Point3 {
        self.origin + t_val * self.direction
    }
}

