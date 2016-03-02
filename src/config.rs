use scattering::modelling::Fields;

#[macro_export]
macro_rules! get_element {
    ($c:ident, $i:expr) => ($c.get($i).unwrap().parse().unwrap();)
}

// impl Plot {
//     pub fn from_config(conf: &Ini) -> Plot {
//         let section = conf.section(Some("plot".to_owned())).unwrap();
//         let low: f64 = get_element!(section, "low");
//         let high: f64 = get_element!(section, "high");
//         let step: f64 = get_element!(section, "step");
//         let var: String = get_element!(section, "var");
//         Plot {
//             low: low,
//             high: high,
//             step: step,
//             var: var,
//             fields: Fields::zero(),
//             n: 0,
//             current: 0,
//         }
//     }
//     pub fn gen_fields(mut self, f: Fields) -> Self {
//         self.fields = f;
//         self
//     }
// }

// impl Fields {
//     pub fn from_config(conf: &Ini) -> Fields {
//         let section = conf.section(Some("fields".to_owned())).unwrap();
//         let E0: Vec2 = get_element!(section, "E0");
//         let E1: Vec2 = get_element!(section, "E1");
//         let E2: Vec2 = get_element!(section, "E2");
//         let B0: f64 = get_element!(section, "B0");
//         let B1: f64 = get_element!(section, "B1");
//         let B2: f64 = get_element!(section, "B2");
//         let omega1: f64 = get_element!(section, "omega1");
//         let omega2: f64 = get_element!(section, "omega2");
//         let phi: f64 = get_element!(section, "phi");
//         Fields::new((E0, E1, E2), (B0, B1, B2), (omega1, omega2), phi)
//     }
// }



// pub struct Probability {
//     pub momentum_error: f64,
//     pub probability_error: f64,
//     pub momentum_samples: usize,
//     pub energy_samples: usize,
//     pub threads: usize,
// }

// impl Probability {
//     pub fn from_config(conf: &Ini) -> Probability {
//         let section = conf.section(Some("probability".to_owned())).unwrap();
//         let momentum_error: f64 = get_element!(section, "momentum_error");
//         let probability_error: f64 = get_element!(section, "probability_error");
//         let momentum_samples: usize = get_element!(section, "momentum_samples");
//         let energy_samples: usize = get_element!(section, "energy_samples");
//         let threads: usize = get_element!(section, "threads");
//         Probability {
//             momentum_error: momentum_error,
//             probability_error: probability_error,
//             momentum_samples: momentum_samples,
//             energy_samples: energy_samples,
//             threads: threads,
//         }
//     }
// }
