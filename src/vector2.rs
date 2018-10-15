use std::ops::{ MulAssign, DivAssign };

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl Vector2 {
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0
        }
    }

    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normalize(&mut self) {
        let len = self.length();
        if len != 0.0 {
            assert_ne!(len, 0.0f64);
            self.x = self.x / len;
            self.y = self.y / len;
        }
    }

    pub fn normalized(&self) -> Self {
        let mut cpy = self.clone();
        cpy.normalize();
        cpy
    }
}

impl MulAssign<f64> for Vector2 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl MulAssign<Vector2> for Vector2 {
    fn mul_assign(&mut self, rhs: Vector2) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl DivAssign<f64> for Vector2 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl DivAssign<Vector2> for Vector2 {
    fn div_assign(&mut self, rhs: Vector2) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}
