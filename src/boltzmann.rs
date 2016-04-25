//! Provides function for creating ensembles of particles with Boltzmann distribution
use std::f64::consts::PI;
use std::cmp::PartialOrd;
use material::Material;
use linal::Vec2;
use rng::Rng;

fn binary_search<T: PartialOrd>(sorted_array: &[T], value: T) -> usize {
    let mut i = 0;
    let mut j = sorted_array.len() - 1;

    while j - i > 1 {
        let m = (i + j) / 2;
        if sorted_array[m] > value {
            j = m;
        } else {
            i = m;
        }
    }

    i
}

#[test]
fn test_binsearch() {
    let mut v1: Vec<f64> = Vec::new();
    let mut v2: Vec<f64> = Vec::new();
    let mut v3: Vec<f64> = Vec::new();
    for i in 1..200 {
        v1.push(i as f64 / 5.0);
        v2.push((i as f64).ln());
        v3.push((i as f64 / 200.0).sin());
    }
    assert_eq!(binary_search(&v1, 12.0) + 1, 60);
    assert_eq!(binary_search(&v2, 5.0) + 1,
               (5.0 as f64).exp().floor() as usize);
    assert_eq!(binary_search(&v3, 0.5) + 1,
               (200.0 * (0.5 as f64).asin()).floor() as usize);
}
/// Boltzmann distribution in polar coordinates:
/// $$ f(p, \theta) = \exp\left[-\frac{\varepsilon(\vec{p}){T}\right]\cdot p $$
fn f<T: Material>(p: f64, theta: f64, temperature: f64, m: &T) -> f64 {
    (-m.energy_polar(p, theta) / temperature).exp() * p
}

fn integrate<F>(f: F, a: f64, b: f64, n: usize) -> (f64, Vec<f64>)
    where F: Fn(f64) -> f64
{
    let mut result: Vec<f64> = vec![0f64; n];
    let step = (b - a) / (n as f64 - 1.0);
    for i in 1..n {
        let x = a + step * (i as f64 - 0.5);
        result[i] = result[i - 1] + step * f(x);
    }
    (result[n - 1], result)
}

pub fn initial_condition<T: Material>(m: &T, temperature: f64, seed: u32, n: usize) -> Vec<Vec2> {
    let mut rng = Rng::new(seed);
    let mut points: Vec<Vec2> = Vec::with_capacity(n);

    let angle_dist_fun = |theta: f64| {
        let dist = |p: f64| f(p, theta, temperature, m);
        let (int, _) = integrate(dist, 0f64, m.brillouin_zone().pmax(theta), 1000);
        int
    };
    
    let (int, angle_dist) = integrate(angle_dist_fun, 0f64, 2.0 * PI, 1000);

    let angle = |x: f64| {
        let i = binary_search(&angle_dist, x * int);
        let w = (x * int - angle_dist[i]) / (angle_dist[i+1] - angle_dist[i]);
        2.0 * PI / (angle_dist.len() as f64 - 1.0) * (i as f64 + w)
    };

    let momentum = |theta: f64, x: f64| {
        let dist = |p: f64| f(p, theta, temperature, m);
        let (int, momentum_dist) = integrate(dist, 0f64, m.brillouin_zone().pmax(theta), 1000);
        let i = binary_search(&momentum_dist, x * int);
        let w = (x * int - momentum_dist[i]) / (momentum_dist[i+1] - momentum_dist[i]);
        m.brillouin_zone().pmax(theta) / (momentum_dist.len() as f64 - 1.0) * (i as f64 + w)
    };

    for _ in 0..n {
        let theta = angle(rng.uniform());
        let p = Vec2::from_polar(momentum(theta, rng.uniform()), theta);
        points.push(p);
    }
    points
}