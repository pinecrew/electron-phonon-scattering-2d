#![allow(dead_code)]
#![allow(non_snake_case)]
use ini::Ini;
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::fs::{File, OpenOptions, remove_file};
use linalg::{Vec2, Point};
use stats::EnsembleStats;

#[macro_export]
macro_rules! get_element {
    ($c:ident, $i:expr) => ($c.get($i).unwrap().parse().unwrap();)
}

pub struct Files {
    pub load_file: bool,
    pub probability: String,
    pub result: String,
}

impl Files {
    pub fn from_config(conf: &Ini) -> Files {
        let section = conf.section(Some("files".to_owned())).unwrap();
        let load_file: bool = get_element!(section, "load_file");
        let probability: String = get_element!(section, "probability");
        let result: String = get_element!(section, "result");
        Files {
            load_file: load_file,
            probability: probability,
            result: result,
        }
    }
    pub fn read_probabilities(&self) -> (Vec<f64>, Vec<f64>) {
        let file = File::open(&self.probability)
                       .ok()
                       .expect(&format!("Can't open {} file", self.probability));
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
    pub fn write_probabilities(&self, energies: &Vec<f64>, probs: &Vec<f64>) {
        let file = File::create(&self.probability)
                       .ok()
                       .expect(&format!("Can't create {} file", self.result));
        let mut writer = BufWriter::new(file);
        let it = energies.iter().zip(probs);
        for (energy, prob) in it {
            write!(writer, "{} {}\n", energy, prob).unwrap();
        }
    }
    pub fn clean_result(&self) {
        let _ = remove_file(&self.result);
    }
    pub fn append_result_line(&self, fields : &Fields, result : &EnsembleStats) {
        let file = OpenOptions::new()
                              .write(true)
                              .append(true)
                              .open(&self.result)
                              .unwrap();
        let mut writer = BufWriter::new(file);
        write!(writer, "{} {} {} {} {} {} {} {} {} ", fields.E.0, fields.E.1, fields.E.2,
               fields.B.0, fields.B.1, fields.B.2, fields.omega.0, fields.omega.1, fields.phi).unwrap();
        write!(writer, "{} {} {} {} {}\n", result.current, result.current_std,
               result.optical, result.acoustic, result.tau).unwrap();
    }
}

pub struct Phonons {
    pub beta: f64,
    pub wla_max: f64,
    pub wlo_max: f64,
    pub T: f64,
}

impl Phonons {
    pub fn from_config(conf : &Ini) -> Phonons {
        let section = conf.section(Some("phonons".to_owned())).unwrap();
        let beta: f64 = get_element!(section, "beta");
        let wla_max: f64 = get_element!(section, "wla_max");
        let wlo_max: f64 = get_element!(section, "wlo_max");
        let T: f64 = get_element!(section, "T");
        Phonons {
            beta: beta,
            wla_max: wla_max,
            wlo_max: wlo_max,
            T: T
        }
    }
}

#[derive(Clone)]
pub struct Fields {
    // E = (E0, E1, E2)
    pub E: (Vec2, Vec2, Vec2),
    // B = (B0, B1, B2)
    pub B: (f64, f64, f64),
    // omega = (omega1, omega2)
    pub omega: (f64, f64),
    pub phi: f64,
}

impl Fields {
    pub fn from_config(conf : &Ini) -> Fields {
        let section = conf.section(Some("fields".to_owned())).unwrap();
        let E0: Vec2 = get_element!(section, "E0");
        let E1: Vec2 = get_element!(section, "E1");
        let E2: Vec2 = get_element!(section, "E2");
        let B0: f64 = get_element!(section, "B0");
        let B1: f64 = get_element!(section, "B1");
        let B2: f64 = get_element!(section, "B2");
        let omega1: f64 = get_element!(section, "omega1");
        let omega2: f64 = get_element!(section, "omega2");
        let phi: f64 = get_element!(section, "phi");
        Fields {
            E: (E0, E1, E2),
            B: (B0, B1, B2),
            omega: (omega1, omega2),
            phi: phi
        }
    }
}


pub struct Bzone {
    pub A: Point,
    pub B: Point,
    pub C: Point,
    pub D: Point,
    pub basis: (Vec2, Vec2),
    pub dual_basis: (Vec2, Vec2),
}

impl Bzone {
    pub fn new(A : Point, B : Point, D : Point) -> Bzone {
        let b = B - A;
        let d = D - A;
        let C = A + b + d;

        let b2 = b - b.ort() * b.dot(d.ort());
        let d2 = d - d.ort() * d.dot(b.ort());

        let b1 = b2 / b.dot(b2);
        let d1 = d2 / d.dot(d2);

        Bzone {
            A: A,
            B: B,
            C: C,
            D: D,
            basis: (b, d),
            dual_basis: (b1, d1)
        }
    }
    pub fn from_config(conf: &Ini) -> Bzone {
        let section = conf.section(Some("bzone".to_owned())).unwrap();
        let A: Point = get_element!(section, "A");
        let B: Point = get_element!(section, "B");
        let D: Point = get_element!(section, "D");
        Bzone::new(A, B, D)
    }
}

pub struct Probability {
    pub momentum_error: f64,
    pub probability_error: f64,
    pub momentum_samples: usize,
    pub energy_samples: usize,
    pub threads: usize
}

impl Probability {
    pub fn from_config(conf: &Ini) -> Probability {
        let section = conf.section(Some("probability".to_owned())).unwrap();
        let momentum_error: f64 = get_element!(section, "momentum_error");
        let probability_error: f64 = get_element!(section, "probability_error");
        let momentum_samples: usize = get_element!(section, "momentum_samples");
        let energy_samples: usize = get_element!(section, "energy_samples");
        let threads: usize = get_element!(section, "threads");
        Probability {
            momentum_error: momentum_error,
            probability_error: probability_error,
            momentum_samples: momentum_samples,
            energy_samples: energy_samples,
            threads: threads
        }
    }
}

pub struct Plot {
    pub low: f64,
    pub high: f64,
    pub step: f64,
    pub var: String,
}

impl Plot {
    pub fn from_config(conf : &Ini) -> Plot {
        let section = conf.section(Some("plot".to_owned())).unwrap();
        let low: f64 = get_element!(section, "low");
        let high: f64 = get_element!(section, "high");
        let step: f64 = get_element!(section, "step");
        let var: String = get_element!(section, "var");
        Plot {
            low: low,
            high: high,
            step: step,
            var: var
        }
    }
    pub fn gen_fields(&self, f: &Fields) -> Vec<Fields> {
        // стоит сделать итератор вместо вектора
        let mut res : Vec<Fields> = Vec::new();
        let n = ((self.high - self.low) / self.step) as usize;
        for i in 0..n {
            let mut fields = f.clone();
            match self.var.as_ref() {
                "E0.x" => fields.E.0.x = self.low + self.step * i as f64,
                "E0.y" => fields.E.0.y = self.low + self.step * i as f64,
                "E1.x" => fields.E.1.x = self.low + self.step * i as f64,
                "E1.y" => fields.E.1.y = self.low + self.step * i as f64,
                "E2.x" => fields.E.2.x = self.low + self.step * i as f64,
                "E2.y" => fields.E.2.y = self.low + self.step * i as f64,
                _ => println!("something went wrong"),
            }
            res.push(fields);
        }
        res
    }
}
