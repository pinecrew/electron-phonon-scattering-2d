use std::f64::consts::PI;
use material::Material;
use linalg::Point;
use rng::Rng;

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
            angle_distrib[i + 1] = angle_distrib[i];
            for j in 0..n {
                let p = pm * j as f64 / (n - 1) as f64;
                angle_distrib[i + 1] += (-m.energy_polar(p, theta) / temperature).exp() * pm /
                                        (n - 1) as f64;
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
        for i in 0..n - 1 {
            let p = pm * i as f64 / (n - 1) as f64;
            dist[i + 1] = dist[i] +
                          (-self.material.energy_polar(p, theta) / self.temperature).exp() * pm /
                          (n - 1) as f64;
        }
        for i in 0..n {
            dist[i] /= dist[n - 1];
        }
        let mut i = 0;
        let mut j = dist.len() - 1;

        while j - i > 1 {
            let m = (i + j) / 2;
            if dist[m] > r {
                j = m;
            } else {
                i = m;
            }
        }
        let w = (r - dist[i]) / (dist[j] - dist[i]);
        let p = (i as f64 + w) / (n - 1) as f64 * pm;
        Point::from_polar(p, theta)
    }
    fn angle(&self, r: f64) -> f64 {
        let mut i = 0;
        let mut j = self.angle_distrib.len() - 1;

        while j - i > 1 {
            let m = (i + j) / 2;
            if self.angle_distrib[m] > r {
                j = m;
            } else {
                i = m;
            }
        }
        let w = (r - self.angle_distrib[i]) / (self.angle_distrib[j] - self.angle_distrib[i]);
        2.0 * PI * (i as f64 + w)
    }
}
