extern crate ini;
use ini::Ini;

extern crate scattering;
use scattering::structs::{Files, Probability, Bzone};
use scattering::linalg::{Vec2, Point};
use scattering::material_specific::{energy, energy_gradient, get_energy_limits};


fn momentums_with_energy_in_dir(e : f64, theta : f64, samples : usize, precision : f64) -> Vec<Point> {
    unimplemented!();
}

fn probability(e : f64, p : &Probability) -> f64 {
    use std::f64::consts::PI;
    use std::cmp::{min,max};

    let mut old : f64 = 0.0;
    let mut new : f64 = 1.0;
    let mut n = 500;
    let mut c = 12;
    while (new-old).abs() / new > p.probability_error && c > 0 {
        old = new;
        new = 0.0;

        let mut prev : Vec<Point> = Vec::new();
        let mut curr : Vec<Point> = Vec::new();

        for i in 0..n {
            let theta = (i as f64) / (n as f64) * 2.0 * PI;
            curr = momentums_with_energy_in_dir(e, theta, p.momentum_samples, p.momentum_error);
            let l = min(curr.len(), prev.len());
            for j in 0..l {
                new += (prev[j] - curr[j]).len() / energy_gradient(&curr[j]).len();
            }
            std::mem::swap(&mut curr, &mut prev);
        }
        n *= 2;
        c -= 1;
    }
    new
}


fn main() {
    let conf = Ini::load_from_file("config.ini").unwrap();

    let prob = Probability::from_config(&conf);
    let files = Files::from_config(&conf);
    let bzone = Bzone::from_config(&conf);

    let (emin, emax) = get_energy_limits(bzone);
    let mut energies: Vec<f64> = Vec::with_capacity(prob.energy_samples);
    let mut probs: Vec<f64> = Vec::with_capacity(prob.energy_samples);
    for i in 0..prob.energy_samples {
        let e = emin + (emin - emax) / (prob.energy_samples as f64 - 1.0) * (i as f64);
        energies.push(e);
    }
    // вот это надо распараллелить
    for e in energies {
        probs.push(probability(e, &prob));
    }
}
