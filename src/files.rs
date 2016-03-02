use std::fs::{File, OpenOptions, remove_file};
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;


use scattering::modelling::Fields;
use scattering::stats::Stats;

pub fn read_probabilities(fname: &str) -> (Vec<f64>, Vec<f64>) {
    let file = File::open(fname)
                   .ok()
                   .expect(&format!("Can't open {} file", fname));
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

pub fn write_probabilities(fname: &str, energies: &Vec<f64>, probs: &Vec<f64>) {
    let file = File::create(fname)
                   .ok()
                   .expect(&format!("Can't create {} file", fname));
    let mut writer = BufWriter::new(file);
    let it = energies.iter().zip(probs);
    for (energy, prob) in it {
        write!(writer, "{} {}\n", energy, prob).unwrap();
    }
}

pub fn clean_result(fname: &str) {
    let _ = remove_file(fname);
}

pub fn append_result_line(fname: &str, fields: &Fields, result: &Stats) {
    let file = OpenOptions::new()
                   .create(true)
                   .write(true)
                   .append(true)
                   .open(fname)
                   .unwrap();
    let mut writer = BufWriter::new(file);
    write!(writer,
           "{} {} {} {} {} {} {} {} {} ",
           fields.e.0,
           fields.e.1,
           fields.e.2,
           fields.b.0,
           fields.b.1,
           fields.b.2,
           fields.omega.1,
           fields.omega.2,
           fields.phi)
        .unwrap();
    write!(writer,
           "{} {} {} {} {}\n",
           result.current,
           result.current_std,
           result.optical,
           result.acoustic,
           result.tau)
        .unwrap();
}
