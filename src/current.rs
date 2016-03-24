extern crate tini;
extern crate time;
extern crate scoped_threadpool;
extern crate scattering;
extern crate linal;

mod material;

use std::env::args;
use std::fs::{OpenOptions, remove_file, create_dir};
use std::io::{BufWriter, Write};
use std::path::Path;

use tini::Ini;
use time::{get_time, SteadyTime};
use scoped_threadpool::Pool;
use scattering::particle::Summary;
use scattering::{Fields, Stats, create_ensemble};
use material::SL;

fn main() {
    let file_name = match args().nth(1) {
        Some(file) => file,
        None => "config.ini".to_owned(),
    };
    let conf = Ini::from_file(&file_name).unwrap();
    let plot = plot_from_config(&conf);
    let fields = fields_from_config(&conf);

    let optical_energy: f64 = conf.get("phonons", "optical_energy").unwrap_or(5e-2);
    let optical_constant: f64 = conf.get("phonons", "optical_constant").unwrap_or(1.5e-3);
    let acoustic_constant: f64 = conf.get("phonons", "acoustic_constant").unwrap_or(1.5e-3);
    let input: String = conf.get("phonons", "input").unwrap_or("data/prob.dat".to_owned());
    let m = SL::with_phonons(optical_energy, optical_constant, acoustic_constant, &input);

    let dt: f64 = conf.get("modelling", "dt").unwrap_or(1e-1);
    let all_time: f64 = conf.get("modelling", "all_time").unwrap_or(1e3);
    let temperature: f64 = conf.get("modelling", "temperature").unwrap_or(7e-3);
    let particles: usize = conf.get("modelling", "particles").unwrap_or(100);
    let threads: usize = conf.get("modelling", "threads").unwrap_or(1);

    let output_clone = plot.output.clone();
    let output = Path::new(&output_clone);
    clean_result(&output);

    println!("start calculations for `{}`", file_name);
    println!("you can find results in `{}`", output.display());

    let plot_count = ((plot.high - plot.low) / plot.step) as u32;
    let all_time_start = SteadyTime::now();

    for (index, f) in plot.domain(&fields).enumerate() {
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

        let mut result = Stats::from_ensemble(&ensemble_summary);
        // dirty: electrons have negative charge
        result.current = -result.current;
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
    low: f64,
    high: f64,
    step: f64,
    var: String,
    output: String,
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
        let value = self.low + self.step * self.current as f64;
        match self.var.as_ref() {
            "E0.x" => fields.e.0.x = value,
            "E0.y" => fields.e.0.y = value,
            "E1.x" => fields.e.1.x = value,
            "E1.y" => fields.e.1.y = value,
            "E2.x" => fields.e.2.x = value,
            "E2.y" => fields.e.2.y = value,
            "B0" => fields.b.0 = value,
            "B1" => fields.b.1 = value,
            "B2" => fields.b.2 = value,
            "phi" => fields.phi = value,
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
    pub fn domain(mut self, f: &Fields) -> Self {
        self.fields = f.clone();
        self
    }
}

fn plot_from_config(conf: &Ini) -> Plot {
    let low: f64 = conf.get("plot", "low").unwrap_or(0.0);
    let high: f64 = conf.get("plot", "high").unwrap_or(0.0);
    let step: f64 = conf.get("plot", "step").unwrap_or(1.0);
    let var: String = conf.get("plot", "var").unwrap_or("E0.y".to_owned());
    let output: String = conf.get("plot", "output").unwrap_or("data/result.dat".to_owned());
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
    let mut f = Fields::zero();
    f.e.0 = conf.get("fields", "E0").unwrap_or(f.e.0);
    f.e.1 = conf.get("fields", "E1").unwrap_or(f.e.1);
    f.e.2 = conf.get("fields", "E2").unwrap_or(f.e.2);
    f.b.0 = conf.get("fields", "B0").unwrap_or(f.b.0);
    f.b.1 = conf.get("fields", "B1").unwrap_or(f.b.1);
    f.b.2 = conf.get("fields", "B2").unwrap_or(f.b.2);
    f.omega.1 = conf.get("fields", "omega1").unwrap_or(f.omega.1);
    f.omega.2 = conf.get("fields", "omega2").unwrap_or(f.omega.2);
    f.phi = conf.get("fields", "phi").unwrap_or(f.phi);
    f
}

fn clean_result(filename: &Path) {
    let _ = remove_file(filename);
}

fn append_result_line(filename: &Path, fields: &Fields, result: &Stats) {
    let parent = filename.parent()
                         .expect(&format!("Can't get parent directory for `{}`", filename.display()));
    if parent.exists() == false {
        create_dir(parent).ok()
                          .expect(&format!("Can't create `{}` directory!", parent.display()));
    }
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
