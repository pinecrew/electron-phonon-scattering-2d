extern crate linalg;

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
use boltzmann::BoltzmannDistrib;

pub fn create_ensemble<T: Material>(n: usize, m: &T, temperature: f64, seed: u32) -> Vec<Particle<T>> {
    let mut rng = Rng::new(seed);
    let bd = BoltzmannDistrib::new(temperature, m);
    let init_condition = bd.make_dist(rng.rand(), n);

    let mut ensemble = Vec::new();

    for ic in init_condition {
        ensemble.push(Particle::new(m, ic, rng.rand()));
    }

    ensemble
}
