extern crate ini;
use ini::Ini;

use std::thread;
use std::sync::{Arc, mpsc};

extern crate scattering;
use scattering::structs::{Files, Probability, Bzone};
use scattering::linalg::{Vec2, Point};
use scattering::material_specific::{energy, energy_gradient, get_energy_limits, pmax};


fn momentums_with_energy_in_dir(e: f64,
                                theta: f64,
                                samples: usize,
                                precision: f64,
                                bzone: &Bzone)
                                -> Vec<Point> {
    let dir = Vec2::from_polar(1.0, theta);
    let step = dir * pmax(theta, bzone) / (samples as f64);

    let mut ps: Vec<Point> = Vec::new();

    for i in 0..samples {
        let mut left = Point::from_vec2(step * i as f64);
        let mut right = left + step;
        if (energy(&left) - e) * (energy(&right) - e) < 0.0 {
            while (right - left).len() > precision {
                let middle = left + (right - left) / 2.0;
                if (energy(&left) - e) * (energy(&middle) - e) < 0.0 {
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

fn probability(e: f64, p: Arc<Probability>, b: Arc<Bzone>) -> f64 {
    use std::f64::consts::PI;
    use std::cmp::{min, max};

    let mut old: f64 = 0.0;
    let mut new: f64 = 1.0;
    let mut n = 500;
    let mut c = 12;
    while (new - old).abs() / new > p.probability_error && c > 0 {
        old = new;
        new = 0.0;

        let mut prev: Vec<Point> = Vec::new();
        let mut curr: Vec<Point> = Vec::new();

        for i in 0..n {
            let theta = (i as f64) / (n as f64) * 2.0 * PI;
            curr = momentums_with_energy_in_dir(e, theta, p.momentum_samples, p.momentum_error, &b);
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

    let prob = Arc::new(Probability::from_config(&conf));
    let files = Files::from_config(&conf);
    let bzone = Arc::new(Bzone::from_config(&conf));

    let (emin, emax) = get_energy_limits(&bzone);
    let mut energies: Vec<f64> = Vec::with_capacity(prob.energy_samples);
    let mut probs: Vec<f64> = Vec::with_capacity(prob.energy_samples);
    for i in 0..prob.energy_samples {
        let e = emin + (emax - emin) / (prob.energy_samples as f64 - 1.0) * (i as f64);
        energies.push(e);
    }
    let mut thread_list = Vec::new();
    let (tx, rx) = mpsc::channel::<f64>();
    for e in &energies {
        let local_tx = tx.clone();
        let local_prob = prob.clone();
        let local_bzone = bzone.clone();
        let local_e = e.clone();
        thread_list.push(thread::spawn(move || {
            let p = probability(local_e, local_prob, local_bzone);
            println!("{} {}", local_e, p); // поглазеть просто
            local_tx.send(p)
                    .ok()
                    .expect("Can't send data");
        }));
    }
    for thread in thread_list {
        let p = rx.recv()
                  .ok()
                  .expect("Can't recv data");
        probs.push(p);
        thread.join()
              .ok()
              .expect("Can't join thread");
    }
    files.write_probabilities(&energies, &probs);
}
