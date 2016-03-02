extern crate ini;
extern crate scoped_threadpool;
extern crate scattering;
extern crate linalg;

mod material;
mod files;

use std::env::args;
use ini::Ini;
use scoped_threadpool::Pool;
use scattering::material::Material;
use scattering::probability::probability;
use material::SL;
use files::write_probabilities;

#[macro_use]
mod config;

fn main() {
    let file_name = args().nth(1)
                          .expect("please input a file!");
    let conf = Ini::load_from_file(&file_name).unwrap();
    let prob = conf.section(Some("probability".to_owned())).unwrap();
    let energy_samples: usize = get_element!(prob, "energy_samples");
    let error: f64 = get_element!(prob, "probability_error");
    let file: String = get_element!(prob, "output");
    let threads: usize = get_element!(prob, "threads");


    let material = SL::new();
    let mut energies: Vec<f64> = Vec::with_capacity(energy_samples);
    let mut probabilities: Vec<f64> = vec![0.0; energy_samples];

    for i in 0..energy_samples {
        let e = material.min_energy() +
                (material.max_energy() - material.min_energy()) /
                (energy_samples as f64 - 1.0) * (i as f64);
        energies.push(e);
    }

    let mut pool = Pool::new(threads as u32);

    pool.scoped(|scope| {
        for (index, item) in probabilities.iter_mut().enumerate() {
            let ref material = material;
            let error = error.clone();
            let energy = energies[index];
            scope.execute(move || {
                *item = probability(energy, material, error);
            });
        }
    });

    write_probabilities(&file, &energies, &probabilities);
}
