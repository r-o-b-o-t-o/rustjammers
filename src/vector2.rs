// Author: Created by Axel

use std::ops::{ Mul, MulAssign, Div, DivAssign, Add, AddAssign, Sub, SubAssign };

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

impl Mul for Vector2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<Vector2> for f64 {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl Mul<f64> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: f64) -> Vector2 {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
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

impl Div for Vector2 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        assert_ne!(rhs.x, 0.0);
        assert_ne!(rhs.y, 0.0);

        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Div<f64> for Vector2 {
    type Output = Vector2;
    fn div(self, rhs: f64) -> Self {
        assert_ne!(rhs, 0.0);

        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<f64> for Vector2 {
    fn div_assign(&mut self, rhs: f64) {
        assert_ne!(rhs, 0.0);

        self.x /= rhs;
        self.y /= rhs;
    }
}

impl DivAssign<Vector2> for Vector2 {
    fn div_assign(&mut self, rhs: Vector2) {
        assert_ne!(rhs.x, 0.0);
        assert_ne!(rhs.y, 0.0);

        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl Add for Vector2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Vector2> for Vector2 {
    fn add_assign(&mut self, rhs: Vector2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vector2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<Vector2> for Vector2 {
    fn sub_assign(&mut self, rhs: Vector2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
