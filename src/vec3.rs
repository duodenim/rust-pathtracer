use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}

impl Vec3 {
    pub fn zero_vector() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {
            x,
            y,
            z
        }
    }
    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn z(&self) -> f32 {
        self.z
    }
    pub fn squared_length(&self) -> f32 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }
    pub fn normalize(&mut self) {
        let length = self.length();
        self.x /= length;
        self.y /= length;
        self.z /= length;
    }
    pub fn unit_vector(v: Vec3) -> Vec3 {
        let mut nv = v;
        nv.normalize();
        nv
    }
    pub fn dot(&self, v2 : Vec3) -> f32 {
        self.x * v2.x + self.y * v2.y + self.z * v2.z
    }
    pub fn cross(&self, v2 : Vec3) -> Vec3 {
        Vec3 {
            x: self.y*v2.z - self.z*v2.y,
            y: -(self.x * v2.z - self.z * v2.x),
            z: self.x*v2.y - self.y*v2.x
        }
    }
    pub fn clamp(&self, min: Vec3, max: Vec3) -> Vec3 {
        let x = if self.x > max.x {
            max.x
        } else if self.x < min.x {
            min.x
        } else {
            self.x
        };
        let y = if self.y > max.y {
            max.y
        } else if self.y < min.y {
            min.y
        } else {
            self.y
        };
        let z = if self.z > max.z {
            max.z
        } else if self.z < min.z {
            min.z
        } else {
            self.z
        };

        Vec3::new(x, y, z)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs
        }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        _rhs * self
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs
        }
    }
}
