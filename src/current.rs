// extern crate ini;
// use ini::Ini;

// extern crate scattering;
// use scattering::modelling::Modelling;

// mod files;

fn main() {
    unimplemented!();
}
// fn main() {
//     let conf = Ini::load_from_file("config.ini").unwrap();

//     clean_result(file_result);
//     for f in plot.gen_fields(fields) {
//         let result = stats();
//         append_result_line(file_result, &f, &result);
//     }
// }



// struct Plot {
//     pub low: f64,
//     pub high: f64,
//     pub step: f64,
//     pub var: String,
//     fields: Fields,
//     n: usize,
//     current: usize,
// }

// impl Iterator for Plot {
//     type Item = Fields;
//     fn next(&mut self) -> Option<Fields> {
//         if self.n == 0 {
//             self.n = ((self.high - self.low) / self.step) as usize;
//         }
//         let mut fields = self.fields.clone();
//         match self.var.as_ref() {
//             "E0.x" => fields.E.0.x = self.low + self.step * self.current as f64,
//             "E0.y" => fields.E.0.y = self.low + self.step * self.current as f64,
//             "E1.x" => fields.E.1.x = self.low + self.step * self.current as f64,
//             "E1.y" => fields.E.1.y = self.low + self.step * self.current as f64,
//             "E2.x" => fields.E.2.x = self.low + self.step * self.current as f64,
//             "E2.y" => fields.E.2.y = self.low + self.step * self.current as f64,
//             _ => {
//                 println!("something went wrong");
//                 return None;
//             }
//         }
//         if self.current < self.n {
//             self.current += 1;
//             Some(fields)
//         } else {
//             None
//         }
//     }
// }



// fn stats() -> Stats {
//     let bd = BolzmannDistrib::new(ph.T, &b);
//     let init_condition = bd.make_dist(self.particles);
//     let mut seed: Vec<u32> = vec![0u32; self.particles];

//     let mut rng = Rng::new(get_time().nsec as u32);
//     for j in 0..self.particles as usize {
//         seed[j] = rng.rand();
//     }

//     let mut ensemble: Vec<ParticleStats> =
//         vec![ParticleStats::new(Vec2::zero(), 0, 0, 0.0); self.particles as usize];
//     let mut pool = Pool::new(self.threads as u32);

//     pool.scoped(|scope| {
//         for (index, item) in ensemble.iter_mut().enumerate() {
//             let f = f.clone();
//             let b = b.clone();
//             let ph = ph.clone();
//             let es = es.clone();
//             let ph = ph.clone();
//             let ic = init_condition[index];
//             let s = seed[index];
//             scope.execute(move || {
//                 *item = self.one_particle(ic, s, &b, &f, &ph, &es, &ps);
//             });
//         }
//     });

//     EnsembleStats::from_ensemble(&ensemble)
// }
