use std::f64::consts::PI;
use std::cmp::PartialOrd;
use material::Material;
use linalg::Point;
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


pub struct BoltzmannDistrib<'a, T: 'a + Material> {
    temperature: f64,
    angle_distrib: Vec<f64>,
    material: &'a T,
}

impl<'a, T: 'a + Material> BoltzmannDistrib<'a, T> {
    pub fn new(temperature: f64, m: &T) -> BoltzmannDistrib<T> {
        let n = 1000;
        let mut angle_distrib: Vec<f64> = vec![0.0; n];
        for i in 0..n - 1 {
            let theta = 2.0 * PI * i as f64 / n as f64;
            let pm = m.brillouin_zone().pmax(theta);
            let step = pm / (n - 1) as f64;
            angle_distrib[i + 1] = angle_distrib[i];
            for j in 0..n {
                let p = step * j as f64;
                angle_distrib[i + 1] += (-m.energy_polar(p, theta) / temperature).exp() * step;
            }
        }
        for i in 0..n {
            angle_distrib[i] /= angle_distrib[n - 1];
        }
        BoltzmannDistrib {
            temperature: temperature,
            angle_distrib: angle_distrib,
            material: m,
        }
    }
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
            dist[i + 1] = dist[i] +
                          (-self.material.energy_polar(p, theta) / self.temperature).exp() * step;
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
