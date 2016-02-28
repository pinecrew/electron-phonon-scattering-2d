#![allow(dead_code)]
#![allow(non_snake_case)]
use ini::Ini;
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::fs::{File, OpenOptions, remove_file};
use linalg::{Vec2, Point};


pub struct Files {
    pub load_file: bool,
    pub probability: String,
    pub result: String,
}

impl Files {
    pub fn from_config(conf: &Ini) -> Files {
        let section = conf.section(Some("files".to_owned())).unwrap();
        let load_file: bool = section.get("load_file").unwrap().parse().unwrap();
        let probability: String = section.get("probability").unwrap().parse().unwrap();
        let result: String = section.get("result").unwrap().parse().unwrap();
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
            let mut data = line.split('\t');
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
        let file = File::create(&self.result)
                       .ok()
                       .expect(&format!("Can't create {} file", self.result));
        let mut writer = BufWriter::new(file);
        let it = energies.iter().zip(probs);
        for (energy, prob) in it {
            write!(writer, "{} {}\n", energy, prob).unwrap();
        }
    }
    pub fn clean_result(&self) {
        remove_file(&self.result)
            .ok()
            .expect(&format!("Can't remove {} file", self.result));
    }
    pub fn append_result_line(&self, fields : &Fields, result : &Res) {
        let file = OpenOptions::new()
                              .write(true)
                              .append(true)
                              .open(&self.result)
                              .unwrap();
        let mut writer = BufWriter::new(file);
        write!(writer, "{} {} {} {} {} {} {} {} {} ", fields.E.0, fields.E.1, fields.E.2,
               fields.B.0, fields.B.1, fields.B.2, fields.omega.0, fields.omega.1, fields.phi).unwrap();
        write!(writer, "{} {} {} {} {}\n", result.current, result.surrent_std,
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
        unimplemented!();
    }
}

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
        unimplemented!();
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
        let A: Point = section.get("A").unwrap().parse().unwrap();
        let B: Point = section.get("B").unwrap().parse().unwrap();
        let D: Point = section.get("D").unwrap().parse().unwrap();
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
        let momentum_error: f64 = section.get("momentum_error").unwrap().parse().unwrap();
        let probability_error: f64 = section.get("probability_error").unwrap().parse().unwrap();
        let momentum_samples: usize = section.get("momentum_samples").unwrap().parse().unwrap();
        let energy_samples: usize = section.get("energy_samples").unwrap().parse().unwrap();
        let threads: usize = section.get("threads").unwrap().parse().unwrap();
        Probability {
            momentum_error: momentum_error,
            probability_error: probability_error,
            momentum_samples: momentum_samples,
            energy_samples: energy_samples,
            threads: threads
        }
    }
}
pub struct Model {
    pub dt: f64,
    pub all_time: f64,
    pub threads: u32,
    pub particles: u32,
}

impl Model {
    pub fn from_config(conf : &Ini) -> Model {
        unimplemented!();
    }
}

pub struct Plot {
    pub low: f64,
    pub hight: f64,
    pub step: f64,
    pub var: String,
}

impl Plot {
    pub fn from_config(conf : &Ini) -> Plot {
        unimplemented!();
    }
    pub fn gen_fields(self, f: &Fields) -> Vec<Fields> {
        // стоит сделать итератор вместо вектора
        unimplemented!();
    }
}

pub struct Res {
    pub current: Vec2,
    pub surrent_std: Vec2,
    pub optical: f64,
    pub acoustic: f64,
    pub tau: f64
}

impl Model {
    pub fn new(dt: f64, all_time: f64, threads: u32, particles: u32) -> Model {
        Model {
            dt: dt,
            all_time: all_time,
            threads: threads,
            particles: particles,
        }
    }
    pub fn run(&self, b : &Bzone, f : &Fields, ph : &Phonons, es : &Vec<f64>, ps : &Vec<f64>) -> Res {
        unimplemented!();
    }
}
