extern crate ini;
use ini::Ini;

fn main() {
    let conf = Ini::load_from_file("config.ini").unwrap();

    let mut section = conf.section(Some("files".to_owned())).unwrap();
    let outfile = section.get("probability").unwrap();

    section = conf.section(Some("probability".to_owned())).unwrap();
    let momentum_error    : f64 = section.get("momentum_error").unwrap().parse().unwrap();
    let momentum_samples  : u32 = section.get("momentum_samples").unwrap().parse().unwrap();
    let probability_error : f64 = section.get("probability_error").unwrap().parse().unwrap();
    let energy_samples    : u32 = section.get("energy_samples").unwrap().parse().unwrap();

    // check it!
    println!("{} {} {} {} {}", outfile, momentum_error, momentum_samples, probability_error, energy_samples);

    // let (emin, emax) = get_energy_limits(energy, bzone);
    // let mut energies: Vec<f64> = Vec::with_capacity(energy_samples);
    // for i in 0..energy_samples {
    //     energies[i] = (emin * (energy_samples - i - 1) + emax * i) / (energy_samples - 1);
    // }
}