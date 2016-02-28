use ini::Ini;
use structs::{Bzone, Fields, Plot, Phonons, Res};

pub struct Model {
    pub dt: f64,
    pub all_time: f64,
    pub threads: u32,
    pub particles: u32,
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
    pub fn from_config(conf : &Ini) -> Model {
        unimplemented!();
    }
    pub fn run(&self, b : &Bzone, f : &Fields, ph : &Phonons, es : &Vec<f64>, ps : &Vec<f64>) -> Res {
        unimplemented!();
    }
}