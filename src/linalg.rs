#![allow(dead_code)]
use std::ops::{Add, Sub, Mul, Div, Neg};
use std::cmp::PartialEq;
use std::fmt;
use std::str::FromStr;


#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x: x, y: y }
    }
    pub fn from_polar(r: f64, theta: f64) -> Vec2 {
        Vec2 { x: r * f64::cos(theta), y: r * f64::sin(theta) }
    }
    pub fn zero() -> Vec2 {
        Vec2::new(0.0, 0.0)
    }
    pub fn dot(self, b: Vec2) -> f64 {
        self.x * b.x + self.y * b.y
    }
    pub fn len(self) -> f64 {
        self.dot(self).sqrt()
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
    pub fn from_polar(r: f64, theta: f64) -> Point {
        Point { x: r * f64::cos(theta), y: r * f64::sin(theta) }
    }
    pub fn zero() -> Point {
        Point::new(0.0, 0.0)
    }
    pub fn from_vec2(v: Vec2) -> Point {
        Point::new(v.x, v.y)
    }
    pub fn position(self) -> Vec2 {
        Vec2::new(self.x, self.y)
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

pub trait Cross<RHS = Self> {
    type Output;
    fn cross(self, rhs: RHS) -> Self::Output;
}

impl Cross<f64> for Vec2 {
    type Output = Self;

    fn cross(self, rhs: f64) -> Self {
        Self::new(self.y, -self.x) * rhs
    }
}

impl Cross for Vec2 {
    type Output = f64;

    fn cross(self, rhs: Vec2) -> f64 {
        self.x * rhs.y - self.y * rhs.x
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
            impl FromStr for $t {
                type Err = ();
                fn from_str(s : &str) -> Result<Self, Self::Err> {
                    let words: Vec<&str> = s.split_whitespace().collect();
                    let x : f64 = words[0].parse().unwrap();
                    let y : f64 = words[1].parse().unwrap();
                    Ok(Self::new(x, y))
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
    fn vector_from_polar() {
        let a = Vec2::new(3.0, 4.0);
        let b = Vec2::from_polar(5.0, f64::atan2(4.0, 3.0));
        assert!((a - b).len() < 1e-10);
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
        assert_eq!(a.dot(b), c);
        assert_eq!(b.dot(a), c);
    }

    #[test]
    fn vector_cross() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(-3.0, 6.0);
        let c = 12.0;
        assert_eq!(a.cross(b), c);
        assert_eq!(b.cross(a), -c);
    }

    #[test]
    fn vector_cross_z() {
        let a = Vec2::new(1.0, 2.0);
        let b = 2.0;
        let c = Vec2::new(4.0, -2.0);
        assert_eq!(a.cross(b), c);
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
