extern crate scattering;
extern crate linal;

use scattering::{Fields, Stats, create_ensemble};
use scattering::material::{Material, BrillouinZone};
use scattering::particle::Summary;
use linal::Vec2;

struct M {
    brillouin_zone: BrillouinZone,
    mass: f64,
}
impl M {
    fn new() -> M {
        let bz = BrillouinZone::new(Vec2::new(-1.0, -1.0),
                                    Vec2::new(1.0, -1.0),
                                    Vec2::new(-1.0, 1.0));
        M {
            brillouin_zone: bz,
            mass: 10.0,
        }
    }
}
impl Material for M {
    fn energy(&self, p: &Vec2) -> f64 {
        let q = *p;
        q.dot(q) / 2.0 / self.mass
    }
    fn energy_gradient(&self, p: &Vec2) -> Vec2 {
        *p / self.mass
    }
    fn velocity(&self, p: &Vec2) -> Vec2 {
        self.energy_gradient(p)
    }
    fn min_energy(&self) -> f64 {
        0.0
    }
    fn max_energy(&self) -> f64 {
        0.1
    }
    fn momentums(&self, energy: f64, theta: f64) -> Vec<Vec2> {
        let mut ms: Vec<Vec2> = Vec::new();
        let pm = self.brillouin_zone.pmax(theta);
        let p = (2.0 * energy * self.mass).sqrt();
        if p < pm {
            ms.push(Vec2::from_polar(p, theta));
        }
        ms
    }
    fn brillouin_zone(&self) -> &BrillouinZone {
        &(self.brillouin_zone)
    }
    fn optical_energy(&self) -> f64 {
        5e-2
    }
    fn optical_scattering(&self, p: &Vec2) -> f64 {
        let mut res = 2.0 * std::f64::consts::PI;
        if self.energy(p) < self.optical_energy() {
            return 0.0;
        }
        let pl = (2.0 * self.mass * (self.energy(p) - self.optical_energy())).sqrt();
        if pl > 1.0 {
            res -= 8.0 * (1.0 / pl).acos();
        }
        1.7e-2 * self.mass * res
    }
    fn acoustic_scattering(&self, p: &Vec2) -> f64 {
        let mut res = 2.0 * std::f64::consts::PI;
        let pl = p.len();
        if pl > 1.0 {
            res -= 8.0 * (1.0 / pl).acos();
        }
        1.7e-2 * self.mass * res
    }
}

fn main() { 
    let ref m = M::new();
    let temperature = 7e-3;
    let ref fields = Fields::zero();
    let n = 36u32;
    let particles = 500usize;
    let mut average = Vec2::zero();
    let mut average_std = Vec2::zero();

    for _ in 0..n {
        let ensemble = create_ensemble(particles, m, temperature);
        for i in &ensemble {
            println!("{} {}", i.init_condition.x, i.init_condition.y);
        }
        let mut ensemble_summary = vec![Summary::empty(); particles];

        for (index, item) in ensemble_summary.iter_mut().enumerate() {
            *item = ensemble[index].run(1e-1, 2e2, fields);
        }

        let result = Stats::from_ensemble(&ensemble_summary);
        average = average + result.current;
        average_std = average_std + result.current_std;
    }
    average = average / (n as f64);
    average_std = average_std / (n as f64) / (n as f64).sqrt();
}
