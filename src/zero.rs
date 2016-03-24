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
use time::get_time;
use scoped_threadpool::Pool;
use scattering::particle::Summary;
use scattering::{Fields, Stats, create_ensemble};
use material::SL;

fn main() {
    let file_name = match args().nth(1) {
        Some(file) => file,
        None => "config.ini".to_owned(),
    };

    let output = Path::new("data/zero-test-new.dat");
    let parent = output.parent()
                       .expect(&format!("Can't get parent directory for `{}`", output.display()));
    if parent.exists() == false {
        create_dir(parent)
            .ok()
            .expect(&format!("Can't create `{}` directory!", parent.display()));
    }

    let conf = Ini::from_file(&file_name).unwrap();
    let optical_energy: f64 = conf.get("phonons", "optical_energy").unwrap();
    let optical_constant: f64 = conf.get("phonons", "optical_constant").unwrap();
    let acoustic_constant: f64 = conf.get("phonons", "acoustic_constant").unwrap();
    let input: String = conf.get("phonons", "input").unwrap();
    let m = SL::with_phonons(optical_energy, optical_constant, acoustic_constant, &input);

    let dt: f64 = conf.get("modelling", "dt").unwrap();
    let all_time: f64 = conf.get("modelling", "all_time").unwrap();
    let temperature: f64 = conf.get("modelling", "temperature").unwrap();
    let particles: usize = conf.get("modelling", "particles").unwrap();
    let threads: usize = conf.get("modelling", "threads").unwrap();

    let f = Fields::zero();
    clean_result(output);
    for _ in 0..100 {
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
        append_result_line(&output, &result);
    }
}


fn clean_result(filename: &Path) {
    let _ = remove_file(filename);
}

fn append_result_line(filename: &Path, result: &Stats) {
    let file = OpenOptions::new()
                   .create(true)
                   .write(true)
                   .append(true)
                   .open(filename)
                   .unwrap();
    let mut writer = BufWriter::new(file);
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
