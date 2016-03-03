#![allow(dead_code)]
#![allow(non_snake_case)]
use ini::Ini;
use linalg::{Vec2, Point};
use stats::EnsembleStats;

#[macro_export]
macro_rules! get_element {
    ($c:ident, $i:expr) => ($c.get($i).unwrap().parse().unwrap();)
}





