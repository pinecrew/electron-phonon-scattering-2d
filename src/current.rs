extern crate ini;
extern crate time;
extern crate scoped_threadpool;
extern crate scattering;
extern crate linalg;
mod files;
#[macro_use]
mod config;
mod material;

use std::env::args;
use ini::Ini;
use time::get_time;
use scoped_threadpool::Pool;
use scattering::modelling::{Modelling, Fields, Summary};
use scattering::stats::Stats;
use scattering::phonons::Phonons;
use scattering::material::Material;
use scattering::boltzmann::BoltzmannDistrib;
use scattering::rng::Rng;
use linalg::Vec2;
use files::{clean_result, append_result_line, read_probabilities};
use material::SL;


fn main() {
    let file_name = match args().nth(1) {
        Some(file) => file,
        None => "config.ini".to_owned(),
    };
    let conf = Ini::load_from_file(&file_name).unwrap();
    let plot = Plot::from_config(&conf);
    let fields = fields_from_config(&conf);
    let phonons = phonons_from_config(&conf);

    let m = SL::new();

    let section = conf.section(Some("modelling".to_owned())).unwrap();
    let dt: f64 = get_element!(section, "dt");
    let all_time: f64 = get_element!(section, "all_time");
    let temperature: f64 = get_element!(section, "temperature");
    let particles: usize = get_element!(section, "particles");
    let threads: usize = get_element!(section, "threads");


    let plot_output = plot.output.clone();
    clean_result(&plot_output);
    for f in plot.gen_fields(&fields) {
        let result = stats(dt,
                           all_time,
                           particles,
                           temperature,
                           &m,
                           &fields,
                           &phonons,
                           threads);
        append_result_line(&plot_output, &f, &result);
    }
}



struct Plot {
    pub low: f64,
    pub high: f64,
    pub step: f64,
    pub var: String,
    pub output: String,
    fields: Fields,
    n: usize,
    current: usize,
}

impl Iterator for Plot {
    type Item = Fields;
    fn next(&mut self) -> Option<Fields> {
        if self.n == 0 {
            self.n = ((self.high - self.low) / self.step) as usize;
        }
        let mut fields = self.fields.clone();
        match self.var.as_ref() {
            "E0.x" => fields.e.0.x = self.low + self.step * self.current as f64,
            "E0.y" => fields.e.0.y = self.low + self.step * self.current as f64,
            "E1.x" => fields.e.1.x = self.low + self.step * self.current as f64,
            "E1.y" => fields.e.1.y = self.low + self.step * self.current as f64,
            "E2.x" => fields.e.2.x = self.low + self.step * self.current as f64,
            "E2.y" => fields.e.2.y = self.low + self.step * self.current as f64,
            _ => {
                println!("something went wrong");
                return None;
            }
        }
        if self.current < self.n {
            self.current += 1;
            Some(fields)
        } else {
            None
        }
    }
}

impl Plot {
    pub fn from_config(conf: &Ini) -> Plot {
        let section = conf.section(Some("plot".to_owned())).unwrap();
        let low: f64 = get_element!(section, "low");
        let high: f64 = get_element!(section, "high");
        let step: f64 = get_element!(section, "step");
        let var: String = get_element!(section, "var");
        let output: String = get_element!(section, "output");
        Plot {
            low: low,
            high: high,
            step: step,
            var: var,
            output: output,
            fields: Fields::zero(),
            n: 0,
            current: 0,
        }
    }
    pub fn gen_fields(mut self, f: &Fields) -> Self {
        self.fields = f.clone();
        self
    }
}


pub fn fields_from_config(conf: &Ini) -> Fields {
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
    Fields::new((E0, E1, E2), (B0, B1, B2), (omega1, omega2), phi)
}

pub fn phonons_from_config(conf: &Ini) -> Phonons {
    let section = conf.section(Some("phonons".to_owned())).unwrap();
    let optical_energy: f64 = get_element!(section, "optical_energy");
    let optical_constant: f64 = get_element!(section, "optical_constant");
    let acoustic_constant: f64 = get_element!(section, "acoustic_constant");
    let fname: String = get_element!(section, "input");
    let (e, p) = read_probabilities(&fname);
    Phonons::new(optical_energy, optical_constant, acoustic_constant, e, p)
}

fn stats<T: Material + Sync>(dt: f64,
                      all_time: f64,
                      particles: usize,
                      temperature: f64,
                      material: &T,
                      fields: &Fields,
                      phonons: &Phonons,
                      threads: usize)
                      -> Stats {
    let mut rng = Rng::new(get_time().nsec as u32);
    let bd = BoltzmannDistrib::new(temperature, material);
    let init_condition = bd.make_dist(rng.rand(), particles);
    let mut seed: Vec<u32> = vec![0u32; particles];

    for j in 0..particles {
        seed[j] = rng.rand();
    }

    let mut ensemble: Vec<Summary> = vec![Summary::new(Vec2::zero(), 0, 0, 0.0); particles];
    let mut pool = Pool::new(threads as u32);

    pool.scoped(|scope| {
        for (index, item) in ensemble.iter_mut().enumerate() {
            let fields = fields;
            let phonons = phonons;
            let ic = init_condition[index];
            let dt = dt;
            let all_time = all_time;
            let s = seed[index];
            let material = material;
            let fields = fields;
            let phonons = phonons;
            scope.execute(move || {
                let particle = Modelling::new(dt, all_time, ic, s);
                *item = particle.run(material, fields, phonons);
            });
        }
    });

    Stats::from_ensemble(&ensemble)
}
