use std::ops::{Add, AddAssign, Sub, Mul, MulAssign, Div, Range};
use std::fmt;
use rand::Rng; 
use rand::thread_rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 {
            e: [e0, e1, e2]
        }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }

    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[1] * other.e[2] - self.e[2] * other.e[1],
                self.e[2] * other.e[0] - self.e[0] * other.e[2],
                self.e[0] * other.e[1] - self.e[1] * other.e[0]
            ]
        }
    }

    pub fn normalized(self) -> Vec3 {
        self / self.length()
    }

    pub fn format_color(self, samples_per_pixel: u64) -> String {
        let ir = (256.0 * (self.e[0] / (samples_per_pixel as f64)).sqrt().clamp(0.0, 0.999)) as u64;
        let ig = (256.0 * (self.e[1] / (samples_per_pixel as f64)).sqrt().clamp(0.0, 0.999)) as u64;
        let ib = (256.0 * (self.e[2] / (samples_per_pixel as f64)).sqrt().clamp(0.0, 0.999)) as u64;

        format!("{} {} {}", ir, ig, ib)
    }


    pub fn random(r: Range<f64>) -> Vec3 {
        let mut rng = thread_rng();
        Vec3 {
            e: [
                rng.gen_range(r.clone()), 
                rng.gen_range(r.clone()),
                rng.gen_range(r.clone()),
            ],
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let v = Vec3::random(-1.0..1.0);
            if v.length() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            (-1.0) * in_unit_sphere
        }
    }

    pub fn near_zero(self) -> bool {
        const EPS: f64 = 1.0e-8;
        self.e[0].abs() < EPS && self.e[1].abs() < EPS && self.e[2].abs() < EPS
    }

    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - 2.0 * self.dot(n) * n
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) -> () {
        *self = Vec3 {
            e: [self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]]
        };
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] - other.e[0],
            self.e[1] - other.e[1],
            self.e[2] - other.e[2],
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, value: f64) -> Vec3 {
        Vec3::new(
            self.e[0] * value,
            self.e[1] * value,
            self.e[2] * value,
        )
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) -> () {
        *self = Vec3 {
            e: [self.e[0] * other, self.e[1] * other, self.e[2] * other]
        };
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self * other.e[0], self * other.e[1], self * other.e[2]]
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Vec3 {
            e: [self.e[0] * other.e[0], self.e[1] * other.e[1], self.e[2] * other.e[2]]
        }
    }
}


impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, value: f64) -> Vec3 {
        self * (1.0 / value)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}
