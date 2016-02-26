#![allow(dead_code)]
use std::ops::{Add, Sub, Mul, Div, Neg};
use std::cmp::PartialEq;

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64
}

impl Vec2 {
    fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x: x, y: y }
    }
    fn zero() -> Vec2 {
        Vec2::new(0.0, 0.0)
    }
    fn dot(&self, _rhs: &Vec2) -> f64 {
        self.x * _rhs.x - self.y * _rhs.y
    }
    fn cross(&self, _rhs: &Vec2) -> f64 {
        self.x * _rhs.y - self.y * _rhs.x
    }
    fn len(&self) -> f64 {
        self.dot(&self).sqrt()
    }
    fn ort(self) -> Vec2 {
        self / self.len()
    }
    fn sqrt(&self) -> Vec2 {
        Vec2::new(self.x.sqrt(), self.y.sqrt())
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Vec2::new(self.x + _rhs.x, self.y + _rhs.y)
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self {
        Vec2::new(self.x - _rhs.x, self.y - _rhs.y)
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec2::new(-self.x, -self.y)
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, _rhs: f64) -> Vec2 {
        Vec2::new(self.x * _rhs, self.y * _rhs)
    }
}

impl Div<f64> for Vec2 {
    type Output = Self;

    fn div(self, _rhs: f64) -> Vec2 {
        if _rhs == 0.0 {
            panic!("Can't divide by zero!");
        }
        Vec2::new(self.x / _rhs, self.y / _rhs)
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
    fn zero() -> Point {
        Point::new(0.0, 0.0)
    }
    fn from_vec2(v: Vec2) -> Point {
        Point::new(v.x, v.y)
    }
}

impl Add<Vec2> for Point {
    type Output = Self;

    fn add(self, _rhs: Vec2) -> Self {
        Point::new(self.x + _rhs.x, self.y + _rhs.y)
    }
}

impl Sub<Vec2> for Point {
    type Output = Self;

    fn sub(self, _rhs: Vec2) -> Self {
        Point::new(self.x - _rhs.x, self.y - _rhs.y)
    }
}

impl Sub for Point {
    type Output = Vec2;

    fn sub(self, _rhs: Point) -> Self::Output {
        Vec2::new(self.x - _rhs.x, self.y - _rhs.y)
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self {
        Point::new(-self.x, -self.y)
    }
}

#[cfg(test)]
mod linalg_test {
    use super::*;

    #[test]
    fn vector_multiply() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 6.0);
        let r = a * 3.0;
        assert_eq!(r, b);
    }

    // insert yourt test
}
