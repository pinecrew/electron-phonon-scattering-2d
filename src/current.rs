extern crate ini;
use ini::Ini;

extern crate scattering;
use scattering::linalg::Vec2;
use scattering::structs::{Files, Bzone};

fn main() {
    let a = Vec2::new(1.0, 2.0);
    let b = Vec2::new(3.0, 4.0);
    let c = a + b;
    println!("Hello Vec2 = {}", c);

    let conf = Ini::load_from_file("config.ini").unwrap();

    let files = Files::from_config(&conf);
    let bzone = Bzone::from_config(&conf);
    let _ = files.read_probabilities();
}
