extern crate linal;
extern crate time;

pub mod material;
pub mod boltzmann;
pub mod particle;
pub mod fields;
pub mod stats;
pub mod probability;
mod rng;

pub use material::Material;
pub use stats::Stats;
pub use fields::Fields;
pub use probability::probability;
use particle::Particle;
use rng::Rng;
use boltzmann::initial_condition;
use time::get_time;

pub fn create_ensemble<T: Material>(n: usize,
                                    m: &T,
                                    temperature: f64)
                                    -> Vec<Particle<T>> {
    let mut rng = Rng::new(get_time().nsec as u32);
    let init_condition = initial_condition(m, temperature, n);

    let mut ensemble = Vec::new();

    for ic in init_condition {
        ensemble.push(Particle::new(m, ic, rng.rand()));
    }

    ensemble
}