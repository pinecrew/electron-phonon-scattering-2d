#![allow(dead_code)]
#![allow(non_snake_case)]
use linalg::{Vec2, Point};

pub struct Files {
    load_file: bool,
    probability: String,
    result: String
}

pub struct Phonons {
    beta: f64,
    wla_max: f64,
    wlo_max: f64,
    T: f64
}

pub struct Fields {
    // E = (E0, E1, E2)
    E: (Vec2, Vec2, Vec2),
    // B = (B0, B1, B2)
    B: (f64, f64, f64),
    // omega = (omega1, omega2)
    omega: (f64, f64),
    // phi = (phi, phi1, phi2)
    phi: (f64, f64, f64)
}

pub struct Bzone {
    A: Point,
    B: Point,
    C: Point,
    D: Point,
    basis: Vec2,
    dual_basis: Vec2
}

pub struct Probability {
    momentum_error: u64,
    probability_error: u64,
    probability: u64,
    energy: u64,
    momentum_samples: i32,
    energy_samples: i32
}

pub struct Model {
    dt: f64,
    all_time: f64,
    threads: u32,
    particles: u32
}

pub struct Plot {
    low: f64,
    hight: f64,
    step: f64,
    var: String
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
