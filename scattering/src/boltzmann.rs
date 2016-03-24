//! Provides function for creating ensembles of particles with Boltzmann distribution
use std::f64::consts::PI;
use std::cmp::PartialOrd;
use material::Material;
use linal::Point;
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
/// Calculates anngle distribution:
/// $$ g(\theta) = A\int\limits\_0^{p_{max}(\theta)}\exp\left[-\frac{\varepsilon(\vec{p}){T}\right]\cdot pdp $$
fn calc_angle_distrib<T: Material>(temperature: f64, m: &T) -> Vec<f64> {
    let n = 1000;
    let mut angle_distrib: Vec<f64> = vec![0.0; n];
    let angle_step = 2.0 * PI / n as f64;
    for i in 0..n - 1 {
        let theta = angle_step * i as f64;
        let pm = m.brillouin_zone().pmax(theta);
        let step = pm / (n - 1) as f64;
        angle_distrib[i + 1] = angle_distrib[i];
        for j in 0..n {
            let p = step * j as f64;
            angle_distrib[i + 1] += f(p, theta, temperature, m) * step * angle_step;
        }
    }
    for i in 0..n {
        angle_distrib[i] /= angle_distrib[n - 1];
    }
    angle_distrib
}


pub struct BoltzmannDistrib<'a, T: 'a + Material> {
    temperature: f64,
    angle_distrib: Vec<f64>,
    material: &'a T,
}

impl<'a, T: 'a + Material> BoltzmannDistrib<'a, T> {
    /// Create new BoltzmannDistrib object for given material and temperature
    pub fn new(temperature: f64, m: &T) -> BoltzmannDistrib<T> {
        BoltzmannDistrib {
            temperature: temperature,
            angle_distrib: calc_angle_distrib(temperature, m),
            material: m,
        }
    }
    /// Make ensemble of n particles with Boltzmann distribution
    pub fn make_dist(&self, seed: u32, n: usize) -> Vec<Point> {
        let mut rng = Rng::new(seed);
        let mut points: Vec<Point> = Vec::with_capacity(n);
        for _ in 0..n {
            let theta = self.angle(rng.uniform());
            let p = self.momentum(theta, rng.uniform());
            points.push(p);
        }
        points
    }
    fn momentum(&self, theta: f64, r: f64) -> Point {
        let n = 1000;
        let mut dist = vec![0.0; n];
        let pm = self.material.brillouin_zone().pmax(theta);
        let step = pm / (n - 1) as f64;
        for i in 0..n - 1 {
            let p = step * i as f64;
            dist[i + 1] = dist[i] + f(p, theta, self.temperature, self.material) * step;
        }
        for i in 0..n {
            dist[i] /= dist[n - 1];
        }
        let i = binary_search(&dist, r);
        let w = (r - dist[i]) / (dist[i + 1] - dist[i]);
        let p = (i as f64 + w) / (n - 1) as f64 * pm;
        Point::from_polar(p, theta)
    }
    fn angle(&self, r: f64) -> f64 {
        let i = binary_search(&self.angle_distrib, r);
        let w = (r - self.angle_distrib[i]) / (self.angle_distrib[i + 1] - self.angle_distrib[i]);
        2.0 * PI * (i as f64 + w)
    }
}

#[test]
fn test_boltzmann() {
    use material::BrillouinZone;
    use linal::{Point, Vec2};

    struct M {
        brillouin_zone: BrillouinZone,
    };
    impl M {
        fn new() -> M {
            let bz = BrillouinZone::new(Point::new(-1.0, -1.0),
                                        Point::new(1.0, -1.0),
                                        Point::new(-1.0, 1.0));
            M { brillouin_zone: bz }
        }
    }
    impl Material for M {
        fn energy(&self, p: &Point) -> f64 {
            let q = p.position();
            q.dot(q) / 20.0
        }
        fn energy_gradient(&self, _: &Point) -> Vec2 {
            unimplemented!();
        }
        fn velocity(&self, _: &Point) -> Vec2 {
            unimplemented!();
        }
        fn min_energy(&self) -> f64 {
            0.0
        }
        fn max_energy(&self) -> f64 {
            0.1
        }
        fn momentums(&self, _: f64, _: f64) -> Vec<Point> {
            unimplemented!();
        }
        fn brillouin_zone(&self) -> &BrillouinZone {
            &(self.brillouin_zone)
        }
        fn optical_energy(&self) -> f64 {
            unimplemented!();
        }
        fn optical_scattering(&self, _: &Point) -> f64 {
            unimplemented!();
        }
        fn acoustic_scattering(&self, _: &Point) -> f64 {
            unimplemented!();
        }
    }
    let ref m = M::new();
    let temperature = 7e-3;
    let bd = BoltzmannDistrib::new(temperature, m);
    let init_condition = bd.make_dist(3_347_183_342u32, 10_000);
    let es: Vec<f64> = init_condition.iter().map(|x| m.energy(x)).collect();
    let intervals = 100;
    let mut dist = vec![0; intervals];
    let width = (m.max_energy() - m.min_energy()) / intervals as f64;
    for e in es {
        let p = ((e - m.min_energy()) / width).floor() as usize;
        dist[p] += 1;
    }
    let x = width / temperature;
    for i in 0..intervals {
        let obtained = dist[i] as f64 / 10_000f64;
        let prob = x * (-(i as f64) * x).exp();
        assert!((obtained - prob).abs() < 3.0 * ((1.0 - prob) * prob / 10_000f64).sqrt());
    }


    let ave = init_condition.iter().fold(Vec2::zero(), |acc, x| acc + x.position()) / 10_000f64;
    assert!(ave.len() < 0.01);
}
