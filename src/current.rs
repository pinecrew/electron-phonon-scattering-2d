extern crate ini;
extern crate time;
extern crate scoped_threadpool;
extern crate scattering;
extern crate linalg;

mod material;

use std::env::args;
use std::fs::{OpenOptions, remove_file};
use std::io::{BufWriter, Write};

use ini::Ini;
use time::{get_time, SteadyTime};
use scoped_threadpool::Pool;
use scattering::particle::Summary;
use scattering::{Fields, Stats, create_ensemble};
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
    let plot = plot_from_config(&conf);
    let fields = fields_from_config(&conf);

    let mut section = conf.section(Some("phonons".to_owned())).unwrap();
    let optical_energy: f64 = get_element!(section, "optical_energy");
    let optical_constant: f64 = get_element!(section, "optical_constant");
    let acoustic_constant: f64 = get_element!(section, "acoustic_constant");
    let input: String = get_element!(section, "input");
    let m = SL::with_phonons(optical_energy, optical_constant, acoustic_constant, &input);

    section = conf.section(Some("modelling".to_owned())).unwrap();
    let dt: f64 = get_element!(section, "dt");
    let all_time: f64 = get_element!(section, "all_time");
    let temperature: f64 = get_element!(section, "temperature");
    let particles: usize = get_element!(section, "particles");
    let threads: usize = get_element!(section, "threads");

    let output = plot.output.clone();
    clean_result(&output);

    println!("start calculations for `{}`", file_name);
    println!("you can find results in `{}`", output);

    let plot_count = ((plot.high - plot.low) / plot.step) as u32;
    let all_time_start = SteadyTime::now();

    for (index, f) in plot.gen_fields(&fields).enumerate() {
        let part_time_start = SteadyTime::now();
        let ensemble = create_ensemble(particles, &m, temperature, get_time().nsec as u32);

        let mut ensemble_summary = vec![Summary::empty(); particles];
        let mut pool = Pool::new(threads as u32);

        pool.scoped(|scope| {
            for (index, item) in ensemble_summary.iter_mut().enumerate() {
                let dt = dt;
                let all_time = all_time;
                let ref fields = f;
                let ref particle = ensemble[index];
                scope.execute(move || {
                    *item = particle.run(dt, all_time, fields);
                });
            }
        });

        let result = Stats::from_ensemble(&ensemble_summary);
        append_result_line(&output, &f, &result);

        let part_time_stop = SteadyTime::now();
        println!(">> point {index} of {count}: done in {time} s",
                 index = index + 1,
                 count = plot_count,
                 time = (part_time_stop - part_time_start).num_seconds());
    }
    let all_time_stop = SteadyTime::now();
    println!(">> total time: {} s",
             (all_time_stop - all_time_start).num_seconds());
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
    pub fn gen_fields(mut self, f: &Fields) -> Self {
        self.fields = f.clone();
        self
    }
}

fn plot_from_config(conf: &Ini) -> Plot {
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

fn fields_from_config(conf: &Ini) -> Fields {
    let section = conf.section(Some("fields".to_owned())).unwrap();
    let mut f = Fields::zero();
    f.e.0 = get_element!(section, "E0");
    f.e.1 = get_element!(section, "E1");
    f.e.2 = get_element!(section, "E2");
    f.b.0 = get_element!(section, "B0");
    f.b.1 = get_element!(section, "B1");
    f.b.2 = get_element!(section, "B2");
    f.omega.1 = get_element!(section, "omega1");
    f.omega.2 = get_element!(section, "omega2");
    f.phi = get_element!(section, "phi");
    f
}

fn clean_result(filename: &str) {
    let _ = remove_file(filename);
}

fn append_result_line(filename: &str, fields: &Fields, result: &Stats) {
    let file = OpenOptions::new()
                   .create(true)
                   .write(true)
                   .append(true)
                   .open(filename)
                   .unwrap();
    let mut writer = BufWriter::new(file);
    write!(writer,
           "{:10.3e} {:10.3e} {:10.3e} {:10.3e} {:10.3e} {:10.3e} {:10.3e} {:10.3e} {:10.3e} \
            {:10.3e} {:10.3e} {:10.3e} ",
           fields.e.0.x,
           fields.e.0.y,
           fields.e.1.x,
           fields.e.1.y,
           fields.e.2.x,
           fields.e.2.y,
           fields.b.0,
           fields.b.1,
           fields.b.2,
           fields.omega.1,
           fields.omega.2,
           fields.phi)
        .unwrap();
    write!(writer,
           "{:10.3e} {:10.3e} {:10.3e} {:10.3e} {:10.3e} {:10.3e} {:10.3e}\n",
           result.current.x,
           result.current_std.x,
           result.current.y,
           result.current_std.y,
           result.optical,
           result.acoustic,
           result.tau)
        .unwrap();
}
