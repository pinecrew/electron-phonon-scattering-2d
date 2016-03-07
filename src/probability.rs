//! Probability is a program for calculation probability of electron-phonon scattering.
//!
//! # Usage
//! ````bash
//! $ probability [config]
//! ````
//! where [config] is a ini-file. If no file is specified `config.ini` is used.
//! This file __must__ include section `[probability]` with following contents:
//!
//! * output -- path to output file
//! * energy_samples -- path to output file
//! * probability_error -- relative error of probability
//! * threads -- number of threads

extern crate ini;
extern crate scoped_threadpool;
extern crate scattering;
extern crate linalg; // need for material

mod material;

use std::env::args;
use std::fs::File;
use std::io::{BufWriter, Write};

use ini::Ini;
use scoped_threadpool::Pool;
use scattering::{Material, probability};
use material::SL;

macro_rules! get_element {
    ($c:ident, $i:expr) => ($c.get($i).unwrap().parse().unwrap();)
}


fn main() {
    let file_name = match args().nth(1) {
        Some(file) => file,
        None => "config.ini".to_owned(),
    };

    let conf = Ini::load_from_file(&file_name).unwrap();
    let section = conf.section(Some("probability".to_owned())).unwrap();

    let energy_samples: usize = get_element!(section, "energy_samples");
    let error: f64 = get_element!(section, "probability_error");
    let output: String = get_element!(section, "output");
    let threads: usize = get_element!(section, "threads");


    let material = SL::without_phonons();
    let mut energies: Vec<f64> = Vec::with_capacity(energy_samples);
    let mut probabilities = vec![0f64; energy_samples];

    let energy_step = (material.max_energy() - material.min_energy()) /
                      (energy_samples as f64 - 1.0);
    for i in 0..energy_samples {
        let e = material.min_energy() + energy_step * (i as f64);
        energies.push(e);
    }

    let mut pool = Pool::new(threads as u32);

    pool.scoped(|scope| {
        for (index, item) in probabilities.iter_mut().enumerate() {
            let ref material = material;
            let error = error;
            let energy = energies[index];
            scope.execute(move || {
                *item = probability(energy, material, error);
            });
        }
    });

    write_probabilities(&output, &energies, &probabilities);
}

fn write_probabilities(filename: &str, energies: &Vec<f64>, probabilities: &Vec<f64>) {
    let file = File::create(filename)
                   .ok()
                   .expect(&format!("Can't create `{}`", filename));
    let mut writer = BufWriter::new(file);
    let data = energies.iter().zip(probabilities);
    for (e, p) in data {
        write!(writer, "{:10.3e} {:10.3e}\n", e, p).unwrap();
    }
}
