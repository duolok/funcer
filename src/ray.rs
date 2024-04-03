use crate::vec3::Vec3;

pub type Point3 = Vec3;

#[derive(Debug)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Point3 {
    pub fn new(origin: &Point3, direction: &Vec3 ) -> Self {
        Self {
            origin, 
            direction
        }
    }

    pub fn origin(&self) {
        self.origin
    }

    pub fn direction(&self) {
        self.direction
    }

    pub fn at(t_val: f64) -> f64 {
        self.origin + t_val * self.direction
    }
}

