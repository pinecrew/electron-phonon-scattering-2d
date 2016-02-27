#![allow(dead_code)]
#![allow(non_snake_case)]
use ini::Ini;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use linalg::{Vec2, Point};


pub struct Files {
    pub load_file: bool,
    pub probability: String,
    pub result: String
}

impl Files {
    pub fn from_config(conf : &Ini) -> Files {
        let section = conf.section(Some("files".to_owned())).unwrap();
        let load_file   : bool = section.get("load_file").unwrap().parse().unwrap();
        let probability : String = section.get("probability").unwrap().parse().unwrap();
        let result      : String = section.get("result").unwrap().parse().unwrap();
        Files {
            load_file: load_file,
            probability: probability,
            result: result
        }
    }
    pub fn read_file(&self) -> (Vec<f64>, Vec<f64>) {
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
                          .expect("Can't parse string")
            );
            b.push(second.parse::<f64>()
                          .ok()
                          .expect("Can't parse string")
            );
        }
        (a, b)
    }
}

pub struct Phonons {
    pub beta: f64,
    pub wla_max: f64,
    pub wlo_max: f64,
    pub T: f64
}

pub struct Fields {
    // E = (E0, E1, E2)
    pub E: (Vec2, Vec2, Vec2),
    // B = (B0, B1, B2)
    pub B: (f64, f64, f64),
    // omega = (omega1, omega2)
    pub omega: (f64, f64),
    // phi = (phi, phi1, phi2)
    pub phi: f64
}

pub struct Bzone {
    pub A: Point,
    pub B: Point,
    pub C: Point,
    pub D: Point,
    pub basis: (Vec2, Vec2),
    pub dual_basis: (Vec2, Vec2)
}

pub struct Probability {
    pub momentum_error: f64,
    pub probability_error: f64,
    pub momentum_samples: i32,
    pub energy_samples: i32
}

impl Probability {
    pub fn from_config(conf : &Ini) -> Probability {
        let section = conf.section(Some("probability".to_owned())).unwrap();
        let momentum_error    : f64 = section.get("momentum_error").unwrap().parse().unwrap();
        let probability_error : f64 = section.get("probability_error").unwrap().parse().unwrap();
        let momentum_samples  : i32 = section.get("momentum_samples").unwrap().parse().unwrap();
        let energy_samples    : i32 = section.get("energy_samples").unwrap().parse().unwrap();
        Probability {
            momentum_error: momentum_error,
            probability_error: probability_error,
            momentum_samples: momentum_samples,
            energy_samples: energy_samples
        }
    }
}
pub struct Model {
    pub dt: f64,
    pub all_time: f64,
    pub threads: u32,
    pub particles: u32
}

pub struct Plot {
    pub low: f64,
    pub hight: f64,
    pub step: f64,
    pub var: String
}

pub struct ExecuteModel {
    phonons: Phonons,
    bzone: Bzone,
    model: Model
}

impl Plot {
    fn gen_model(field: &Fields) -> Model {
        unimplemented!()
    }
}

impl Model {
    fn new(dt: f64, all_time: f64, threads: u32, particles: u32) -> Model {
        Model {
            dt: dt,
            all_time: all_time,
            threads: threads,
            particles: particles
        }
    }
    fn null() -> Model {
        Model {
            dt: 0.0,
            all_time: 0.0,
            threads: 0,
            particles: 0
        }
    }
}

impl ExecuteModel {
    fn new(phonons: Phonons, bzone: Bzone) -> ExecuteModel {
        ExecuteModel {
            phonons: phonons,
            bzone: bzone,
            model: Model::null(),
        }
    }
    fn set_model(&mut self, model: Model) {
        self.model = model;
    }
    fn run(&self) {
        unimplemented!()
    }
}
