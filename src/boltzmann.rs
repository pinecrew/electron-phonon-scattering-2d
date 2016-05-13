//! Provides function for creating ensembles of particles with Boltzmann distribution
use std::f64::consts::PI;
use std::cmp::PartialOrd;
use material::Material;
use linal::Vec2;
use rng::Rng;
use time::get_time;

pub fn initial_condition<T: Material>(m: &T, temperature: f64, n: usize) -> Vec<Vec2> {
    let seed = get_time().nsec as u32;
    let mut rng = Rng::new(seed);
    let mut points: Vec<Vec2> = Vec::with_capacity(n);

    let mut k = n;
    let bz = m.brillouin_zone();
    let (a, b) = bz.basis;
    while k > 0 {
        let p = bz.a + a * rng.uniform() + b * rng.uniform();

        if (rng.uniform() < ((m.min_energy() - m.energy(p)) / temperature).exp()) {
            points.push(p);
            k -= 1;
        }
    }   
    points
}

#[cfg(test)]
mod tests {
    use super::*;
    use material::*;
    use stats::{Mean, MeanStd};
    
    extern crate linal;
    use linal::Vec2;


    struct Parabolic {
        mass: f64,
        bz: BrillouinZone,
    }

    impl Parabolic {
        pub fn new() -> Parabolic {
            let bz = BrillouinZone::new(Vec2::new(-1, -1), Vec2::new(1, -1), Vec2::new(-1, 1));
            Parabolic {
                mass: 1.0,
                bz: bz,
            }
        }
    }

    impl Material for Parabolic {
        fn energy(&self, p: Vec2) -> f64 {
            p.dot(p) / 2.0
        }
        /// Gradient of energy in momentum space
        fn energy_gradient(&self, p: Vec2) -> Vec2 {
            unimplemented!();
        }

        fn velocity(&self, p: Vec2) -> Vec2 {
            unimplemented!();
        }
        /// Minimum of energy in brillouin zone
        fn min_energy(&self) -> f64 {
            0f64
        }
        /// Maximum of energy in brillouin zone
        fn max_energy(&self) -> f64 {
            1f64
        }
        /// Solves equation energy_polar(p, theta) = energy
        fn momentums(&self, energy: f64, theta: f64) -> Vec<Vec2> {
            unimplemented!();
        }
        /// brillouin zone structure
        fn brillouin_zone(&self) -> &BrillouinZone {
            &self.bz
        }
        /// optical phonon energy
        fn optical_energy(&self) -> f64 {
            unimplemented!();
        }
        /// optical phonon scattering probability
        fn optical_scattering(&self, p: Vec2) -> f64 {
            unimplemented!();
        }
        /// acoustic phonon scattering probability
        fn acoustic_scattering(&self, p: Vec2) -> f64 {
            unimplemented!();
        }
    }

    fn is_ok(real: Vec2, expectation: Vec2, delta: Vec2) -> bool {
        (real.x - expectation.x).abs() < delta.x && (real.y - expectation.y).abs() < delta.y
    }

    #[test]
    fn test_average_momentum() {
        let ref m = Parabolic::new();
        let temperature = 0.05;
        let ic = initial_condition(m, temperature, 1000000usize);
        let average = ic.mean();
        let std = ic.mean_std();
        assert!(average.x.abs() < std.x, format!("{} > {}", average.x.abs(), std.x));
        assert!(average.y.abs() < std.y, format!("{} > {}", average.y.abs(), std.y));
    }

    #[test]
    fn test_average_energy() {
        let ref m = Parabolic::new();
        let temperature = 0.05;
        let ic = initial_condition(m, temperature, 1000000usize);
        let energies: Vec<f64> = ic.iter().map(|&x| m.energy(x)).collect();
        let average = energies.mean();
        let std = energies.mean_std();
        assert!((average - temperature).abs() < std * 2.0, format!("{} > {}", (average - temperature).abs(), 2.0*std));
    }
}
