extern crate ini;
use ini::Ini;

extern crate scattering;
use scattering::linalg::Vec2;
use scattering::structs::{Files, Bzone};

fn main() {
    let conf = Ini::load_from_file("config.ini").unwrap();
    let files = Files::from_config(&conf);
    let bzone = Bzone::from_config(&conf);
    let fields = Field::from_config(&conf);
    let plot = Plot::from_config(&conf);
    let _ = files.read_probabilities();
}
