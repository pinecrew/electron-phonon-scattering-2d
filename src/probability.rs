extern crate ini;
use ini::Ini;

extern crate scattering;
use scattering::structs::{Files, Probability};

fn main() {
    let conf = Ini::load_from_file("config.ini").unwrap();
    let prob = Probability::from_config(&conf);
    let files = Files::from_config(&conf);

    // check it!
    println!("{} {} {} {} {}", files.probability, prob.momentum_error,
     prob.momentum_samples, prob.probability_error, prob.energy_samples);
    let _ = files.read_file();

    // let (emin, emax) = get_energy_limits(energy, bzone);
    // let mut energies: Vec<f64> = Vec::with_capacity(energy_samples);
    // for i in 0..energy_samples {
    //     energies.push((emin * (energy_samples - i - 1) + emax * i) / (energy_samples - 1));
    // }
}
