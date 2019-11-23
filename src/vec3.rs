use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn x(self) -> f32{
        self.x
    }

    pub fn y(self) -> f32{
        self.y
    }

    pub fn z(self) -> f32{
        self.z
    }

    pub fn squared_length(self) -> f32 {
        self.x().powf(2.0) + self.y().powf(2.0) + self.z().powf(2.0)
    }

    pub fn length(self) -> f32 {
        (self.squared_length()).sqrt()
    }

    #[allow(dead_code)]
    pub fn unit(self) -> Vec3 {
        self / self.length()
    }

    pub fn dot(self, v: Vec3) -> f32 {
        self.x() * v.x() + self.y() * v.y() + self.z() * v.z()
    }

    #[allow(dead_code)]
    pub fn cross(self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.y() * v.z() - self.z() * v.y(),
            self.z() * v.x() - self.x() * v.z(),
            self.x() * v.y() - self.y() * v.x(),
        )
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() + v.x(), self.y() + v.y(), self.z() + v.z())
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() / v.x(), self.y() / v.y(), self.z() / v.z())
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, f: f32) -> Vec3 {
        Vec3::new(self.x() / f, self.y() / f, self.z() / f)
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, t: f32) {
        let k = 1.0 / t;
        self.x *= k;
        self.y *= k;
        self.z *= k;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() - v.x(), self.y() - v.y(), self.z() - v.z())
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() * v.x(), self.y() * v.y(), self.z() * v.z())
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, f: f32) -> Vec3 {
        Vec3::new(self.x() * f, self.y() * f, self.z() * f)
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3{
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}