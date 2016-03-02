use scattering::material::{Material, BrillouinZone};
use linalg::{Point, Vec2};
use std::f64::consts::PI;

const VF: f64 = 1e6;
const HBAR: f64 = 6.5e-16;
const C: f64 = 3e8;
const D: f64 = 2e-8;
const EPS0: f64 = 0.059;
const EPS1: f64 = 0.029;
const G: f64 = (EPS1 / EPS0) * (EPS1 / EPS0);
const A: f64 = VF * HBAR / EPS0 / D;


pub struct SL {
    minimum_energy: f64,
    maximum_energy: f64,
    brillouin_zone: BrillouinZone,
}

impl SL {
    pub fn new() -> SL {
        let a = Point::new(-30.0, -PI);
        let b = Point::new(30.0, -PI);
        let d = Point::new(-30.0, PI);
        let brillouin_zone = BrillouinZone::new(a, b, d);
        let mut s = SL {
            minimum_energy: 0.0,
            maximum_energy: 0.0,
            brillouin_zone: brillouin_zone,
        };
        let (min, max) = s.get_energy_limits();
        s.minimum_energy = min;
        s.maximum_energy = max;
        s
    }

    fn get_energy_limits(&self) -> (f64, f64) {
        let mut emin = 100.0;
        let mut emax = -100.0;
        let n = 1000;
        for i in 0..n {
            for j in 0..n {
                let i1 = (i as f64) / (n as f64);
                let j1 = (j as f64) / (n as f64);
                let p = self.brillouin_zone.a + self.brillouin_zone.basis.0 * i1 +
                        self.brillouin_zone.basis.1 * j1;
                let e = self.energy(&p);
                if e < emin {
                    emin = e;
                }
                if e > emax {
                    emax = e;
                }
            }
        }
        (emin, emax)
    }
}

impl Material for SL {
    // Выражение для энергетического спектра (в декартовых координатах)
    fn energy(&self, p: &Point) -> f64 {
        let root = (1.0 + A * A * p.x * p.x).sqrt();
        EPS0 * (root + G * (1.0 - p.y.cos()) / root)
    }


    // Градиент энергии в импульсном пространстве
    fn energy_gradient(&self, p: &Point) -> Vec2 {
        let b = 1.0 + A * A * p.x * p.x;
        let root = b.sqrt();
        Vec2::new(EPS0 * A * A * p.x / root * (1.0 - G * (1.0 - p.x.cos()) / b),
                  G * EPS0 / root * p.y.sin())
    }

    // Скорость
    fn velocity(&self, p: &Point) -> Vec2 {
        self.energy_gradient(&p) * (D / HBAR / C)
    }

    fn momentums(&self, e: f64, theta: f64) -> Vec<Point> {
        let samples = 20;
        let precision = 1e-5;
        let dir = Vec2::from_polar(1.0, theta);
        let step = dir * self.brillouin_zone().pmax(theta) / (samples as f64);

        let mut ps: Vec<Point> = Vec::new();

        for i in 0..samples {
            let mut left = Point::from_vec2(step * i as f64);
            let mut right = left + step;
            if (self.energy(&left) - e) * (self.energy(&right) - e) < 0.0 {
                while (right - left).len() > precision {
                    let middle = left + (right - left) / 2.0;
                    if (self.energy(&left) - e) * (self.energy(&middle) - e) < 0.0 {
                        right = middle;
                    } else {
                        left = middle;
                    }
                }

                ps.push(left + (right - left) / 2.0);
            }
        }
        ps
    }

    fn min_energy(&self) -> f64 {
        self.minimum_energy
    }
    fn max_energy(&self) -> f64 {
        self.maximum_energy
    }
    fn brillouin_zone(&self) -> &BrillouinZone {
        &(self.brillouin_zone)
    }
}