#![allow(dead_code)]
use std::ops::{Add, Sub, Mul, Div, Neg};
use std::cmp::PartialEq;
use std::fmt;

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
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x: x, y: y }
    }
    pub fn zero() -> Vec2 {
        Vec2::new(0.0, 0.0)
    }
    pub fn dot(a: Vec2, b: Vec2) -> f64 {
        a.x * b.x + a.y * b.y
    }
    pub fn cross(a: Vec2, b: Vec2) -> f64 {
        a.x * b.y - a.y * b.x
    }
    pub fn cross_z(&self, _rhs: f64) -> Vec2 {
        Vec2::new(self.y, -self.x) * _rhs
    }
    pub fn len(&self) -> f64 {
        Vec2::dot(*self, *self).sqrt()
    }
    pub fn ort(self) -> Vec2 {
        self / self.len()
    }
    pub fn sqrt(&self) -> Vec2 {
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

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
    pub fn zero() -> Point {
        Point::new(0.0, 0.0)
    }
    pub fn from_vec2(v: Vec2) -> Point {
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

macro_rules! same_functional_impl {
    ( $( $t:ty ),* ) => {
        $(
            impl Neg for $t {
                type Output = Self;

                fn neg(self) -> Self {
                    Self::new(-self.x, -self.y)
                }
            }
            impl PartialEq for $t {
                fn eq(&self, other: &Self) -> bool {
                    self.x == other.x && self.y == other.y
                }
            }
            impl fmt::Display for $t {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "({}, {})", self.x, self.y)
                }
            }
        )*
    }
}

same_functional_impl!(Vec2, Point);

#[cfg(test)]
mod linalg_test {
    use super::*;

    #[test]
    fn vector_mul() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 6.0);
        let r = a * 3.0;
        assert_eq!(r, b);
    }

    #[test]
    #[should_panic]
    fn vector_div() {
        let a = Vec2::new(1.0, 2.0);
        let _ = a / 0.0;
    }

    #[test]
    fn vector_add() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(-3.0, 6.0);
        let c = Vec2::new(-2.0, 8.0);
        assert_eq!(a+b, c);
    }

    #[test]
    fn vector_sub() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(-3.0, 6.0);
        let c = Vec2::new(4.0, -4.0);
        assert_eq!(a-b, c);
    }

    #[test]
    fn vector_dot() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(-3.0, 6.0);
        let c = 9.0;
        assert_eq!(Vec2::dot(a, b), c);
    }

    #[test]
    fn vector_cross() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(-3.0, 6.0);
        let c = 12.0;
        assert_eq!(Vec2::cross(a, b), c);
    }

    #[test]
    fn vector_cross_z() {
        let a = Vec2::new(1.0, 2.0);
        let b = 2.0;
        let c = Vec2::new(4.0, -2.0);
        assert_eq!(a.cross_z(b), c);
    }

    #[test]
    fn vector_neg() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(-1.0, -2.0);
        assert_eq!(-a, b);
    }

    #[test]
    fn point_vector_add() {
        let a = Point::new(1.0, 2.0);
        let b = Vec2::new(-3.0, 6.0);
        let c = Point::new(-2.0, 8.0);
        assert_eq!(a+b, c);
    }

    #[test]
    fn point_vector_sub() {
        let a = Point::new(1.0, 2.0);
        let b = Vec2::new(-3.0, 6.0);
        let c = Point::new(4.0, -4.0);
        assert_eq!(a-b, c);
    }

    #[test]
    fn point_sub() {
        let a = Point::new(1.0, 2.0);
        let b = Point::new(-3.0, 6.0);
        let c = Vec2::new(4.0, -4.0);
        assert_eq!(a-b, c);
    }

    #[test]
    fn point_neg() {
        let a = Point::new(1.0, 2.0);
        let b = Point::new(-1.0, -2.0);
        assert_eq!(-a, b);
    }
}
