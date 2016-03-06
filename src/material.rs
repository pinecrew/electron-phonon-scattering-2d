use std::fs::File;
use std::io::{BufReader, BufRead};
use std::f64::consts::PI;

use scattering::material::{Material, BrillouinZone};
use linalg::{Point, Vec2};

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

    // phonons:
    optical_energy: f64,
    acoustic_constant: f64,
    optical_constant: f64,
    energies: Vec<f64>,
    probabilities: Vec<f64>,
}

impl SL {
    pub fn without_phonons() -> SL {
        let a = Point::new(-30.0, -PI);
        let b = Point::new(30.0, -PI);
        let d = Point::new(-30.0, PI);
        let brillouin_zone = BrillouinZone::new(a, b, d);
        let mut s = SL {
            minimum_energy: 0.0,
            maximum_energy: 0.0,
            brillouin_zone: brillouin_zone,
            optical_energy: 0.0,
            optical_constant: 0.0,
            acoustic_constant: 0.0,
            energies: Vec::new(),
            probabilities: Vec::new(),
        };
        let (min, max) = s.get_energy_limits();
        s.minimum_energy = min;
        s.maximum_energy = max;
        s
    }
    #[allow(dead_code)]
    pub fn with_phonons(optical_energy: f64,
                        optical_constant: f64,
                        acoustic_constant: f64,
                        fname: &str)
                        -> SL {
        let mut s = SL::without_phonons();
        let (e, p) = read_probabilities(fname);
        s.optical_energy = optical_energy;
        s.optical_constant = optical_constant;
        s.acoustic_constant = acoustic_constant;
        s.energies = e;
        s.probabilities = p;
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

    fn probability(&self, energy: f64) -> f64 {
        let step = self.energies[1] - self.energies[0];

        if energy < self.energies[0] || energy > self.energies[self.energies.len() - 1] {
            return 0.0;
        }

        let pos = (energy - self.energies[0]) / step;
        let i = pos.floor() as usize;
        let w = pos - pos.floor();
        self.probabilities[i] * (1.0 - w) + self.probabilities[i + 1] * w
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
        Vec2::new(EPS0 * A * A * p.x / root * (1.0 - G * (1.0 - p.y.cos()) / b),
                  G * EPS0 / root * p.y.sin())
    }

    // Скорость
    fn velocity(&self, p: &Point) -> Vec2 {
        self.energy_gradient(&p) * (D / HBAR / C)
    }

    fn momentums(&self, e: f64, theta: f64) -> Vec<Point> {
        let samples = 20;
        let precision = 1e-7;
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
    fn optical_energy(&self) -> f64 {
        self.optical_energy
    }
    fn optical_scattering(&self, p: &Point) -> f64 {
        self.optical_constant * self.probability(self.energy(p) - self.optical_energy)
    }
    fn acoustic_scattering(&self, p: &Point) -> f64 {
        self.acoustic_constant * self.probability(self.energy(p))
    }
}


fn read_probabilities(filename: &str) -> (Vec<f64>, Vec<f64>) {
    let file = File::open(filename)
                   .ok()
                   .expect(&format!("Can't open `{}`", filename));
    let reader = BufReader::new(file);
    let (mut a, mut b) = (Vec::new(), Vec::new());
    for line in reader.lines().filter_map(|result| result.ok()) {
        let mut data = line.split_whitespace();
        let first = data.next()
                        .expect("Can't get item");
        let second = data.next()
                         .expect("Can't get item");
        a.push(first.parse::<f64>()
                    .ok()
                    .expect("Can't parse string"));
        b.push(second.parse::<f64>()
                     .ok()
                     .expect("Can't parse string"));
    }
    (a, b)
}
