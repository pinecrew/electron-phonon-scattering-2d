//! Simple library to work with 2D vectors and points

#![warn(missing_docs)]
use std::ops::{Add, Sub, Mul, Div, Neg};
use std::cmp::PartialEq;
use std::fmt;
use std::str::FromStr;
use std::num;

/// 2D vector in cortesian coordinates
#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    /// component of vector
    pub x: f64,
    /// component of vector
    pub y: f64,
}

/// 2D point in cortesian coordinates
#[derive(Debug, Clone, Copy)]
pub struct Point {
    /// component of point
    pub x: f64,
    /// component of point
    pub y: f64,
}

impl Vec2 {
    /// Constructs a new `Vec2`.
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x: x, y: y }
    }
    /// Constructs a new `Vec2` from polar coordinates $(r, \theta)$.
    pub fn from_polar(r: f64, theta: f64) -> Vec2 {
        Vec2 {
            x: r * f64::cos(theta),
            y: r * f64::sin(theta),
        }
    }
    /// Create a zero `Vec2`
    pub fn zero() -> Vec2 {
        Vec2::new(0.0, 0.0)
    }
    /// Scalar product
    pub fn dot(self, b: Vec2) -> f64 {
        self.x * b.x + self.y * b.y
    }
    /// Vector length
    pub fn len(self) -> f64 {
        self.dot(self).sqrt()
    }
    /// Unary vector, co-directed with given
    pub fn ort(self) -> Vec2 {
        self / self.len()
    }
    /// Squares of the vector coordinates
    pub fn sqr(&self) -> Vec2 {
        Vec2::new(self.x.powi(2), self.y.powi(2))
    }
    /// Square root of vector coordinates
    pub fn sqrt(&self) -> Vec2 {
        Vec2::new(self.x.sqrt(), self.y.sqrt())
    }
}
/// Constructs dual basis for given.
///
/// Dual basis $(b_1, b_2)$ for basis $(a_1, a_2)$ satisfies relation
/// $$a_i \cdot b_j = \delta_{ij}$$
///
/// # Example
/// ```
/// use linalg::{Vec2, dual_basis};
///
/// let a1 = Vec2::new(2.0, 0.0);
/// let a2 = Vec2::new(3.0, 4.0);
///
/// let (b1, b2) = dual_basis((a1, a2));
/// assert_eq!(b1, Vec2::new(0.5, -0.375));
/// assert_eq!(b2, Vec2::new(0.0, 0.25));
/// ```
pub fn dual_basis(basis: (Vec2, Vec2)) -> (Vec2, Vec2) {
    let (a, b) = basis;

    // At first, construct vectors a1 and b1 such that
    // a1.dot(b) = 0 и b1.dot(a) = 0
    let a1 = a - b * a.dot(b) / b.dot(b);
    let b1 = b - a * b.dot(a) / a.dot(a);

    // And second, normalize them
    let a2 = a1 / a.dot(a1);
    let b2 = b1 / b.dot(b1);

    (a2, b2)
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

impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, _rhs: Vec2) -> Vec2 {
        Vec2::new(self.x * _rhs.x, self.y * _rhs.y)
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
    /// Constructs a new `Point`
    ///
    /// #Examples
    ///
    /// ```
    /// use linalg::Point;
    /// // ..
    /// // create `Point` with coords (1.5, 3.4)
    /// let a: Point = Point::new(1.5, 3.4);
    /// // return: a = 1.5 3.4
    /// println!("a = {}", a);
    /// ```
    pub fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
    /// Constructs a new `Point` from polar coordinates $(r, \theta)$.
    pub fn from_polar(r: f64, theta: f64) -> Point {
        Point {
            x: r * f64::cos(theta),
            y: r * f64::sin(theta),
        }
    }
    /// Constructs a zero `Point`
    ///
    /// #Examples
    ///
    /// ```
    /// use linalg::Point;
    /// // ...
    /// // create a zero `Point`
    /// let a = Point::zero();
    /// // create b `Point`
    /// let b = Point::new(0.0, 0.0);
    /// // a == b
    /// assert_eq!(a, b)
    /// ```
    pub fn zero() -> Point {
        Point::new(0.0, 0.0)
    }
    /// Construct `Point` from given `Vec2`
    pub fn from_vec2(v: Vec2) -> Point {
        Point::new(v.x, v.y)
    }
    /// ?
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
                    write!(f, "{} {}", self.x, self.y)
                }
            }
            impl FromStr for $t {
                type Err = num::ParseFloatError;
                fn from_str(s : &str) -> Result<Self, Self::Err> {
                    let words: Vec<&str> = s.split_whitespace().collect();
                    let x : f64 = try!(words[0].parse());
                    let y : f64 = try!(words[1].parse());
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
        assert_eq!(a + b, c);
    }

    #[test]
    fn vector_sub() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(-3.0, 6.0);
        let c = Vec2::new(4.0, -4.0);
        assert_eq!(a - b, c);
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
        assert_eq!(a + b, c);
    }

    #[test]
    fn point_vector_sub() {
        let a = Point::new(1.0, 2.0);
        let b = Vec2::new(-3.0, 6.0);
        let c = Point::new(4.0, -4.0);
        assert_eq!(a - b, c);
    }

    #[test]
    fn point_sub() {
        let a = Point::new(1.0, 2.0);
        let b = Point::new(-3.0, 6.0);
        let c = Vec2::new(4.0, -4.0);
        assert_eq!(a - b, c);
    }

    #[test]
    fn point_neg() {
        let a = Point::new(1.0, 2.0);
        let b = Point::new(-1.0, -2.0);
        assert_eq!(-a, b);
    }

    #[test]
    fn vector_parse() {
        let a: Vec2 = "1 2".parse().unwrap();
        assert_eq!(a, Vec2::new(1.0, 2.0));
    }
}
