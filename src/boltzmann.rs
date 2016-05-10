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

        if (rng.uniform() < ((m.min_energy() - m.energy(&p)) / temperature).exp()) {
            points.push(p);
            k -= 1;
        }
    }
    points
}
