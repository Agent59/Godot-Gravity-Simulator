use std::ops;

use crate::base::Scalar;

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: Scalar,
    pub y: Scalar,
}

impl Vec2 {
    pub fn new(x: Scalar, y: Scalar) -> Self {
        Self {x, y}
    }

    pub fn length(&self) -> Scalar {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vec2 {
        Vec2::new(self.x / self.length(), self.y / self.length())
    }
}

impl ops::Add<Self> for Vec2 {
    type Output = Self;
    fn add(self, vec2: Vec2) -> Self::Output {
        Vec2::new(self.x + vec2.x, self.y + vec2.y)
    }
}

impl ops::AddAssign<Self> for Vec2 {
    fn add_assign(&mut self, vec2: Vec2) {
        self.x += vec2.x; self.y += vec2.y;
    }
}

impl ops::Sub<Self> for Vec2 {
    type Output = Self;
    fn sub(self, vec2: Vec2) -> Self::Output {
        Vec2::new(self.x - vec2.x, self.y - vec2.y)
    }
}

impl ops::SubAssign<Self> for Vec2 {
    fn sub_assign(&mut self, vec2: Vec2) {
        self.x =- vec2.x; self.y =- vec2.y;
    }
}

impl ops::Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec2::new(-self.x, -self.y)
    }
}

impl ops::Mul<Scalar> for Vec2 {
    type Output = Self;
    fn mul(self, scalar: Scalar) -> Self::Output {
        Vec2::new(self.x * scalar, self.y * scalar)
    }
}

impl ops::Mul<Vec2> for Scalar {
    type Output = Vec2;
    fn mul(self, vec2: Vec2) -> Self::Output {
        Vec2::new(self * vec2.x, self * vec2.y)
    }
}

impl ops::MulAssign<Scalar> for Vec2 {
    fn mul_assign(&mut self, scalar: Scalar) {
        self.x *= scalar; self.y *= scalar;
    }
}

impl ops::Div<Scalar> for Vec2 {
    type Output = Self;
    fn div(self, scalar: Scalar) -> Self::Output {
        Vec2::new(self.x / scalar, self.y / scalar)
    }
}

impl ops::DivAssign<Scalar> for Vec2 {
    fn div_assign(&mut self, scalar: Scalar) {
        self.x /= scalar; self.y /= scalar;
    }
}

impl core::cmp::PartialEq<Vec2> for Vec2 {
    fn eq(&self, vec2: &Vec2) -> bool {
        self.x == vec2.x && self.y == vec2.y
    }
}

impl Eq for Vec2 {}
