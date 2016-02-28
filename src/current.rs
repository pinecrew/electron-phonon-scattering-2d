extern crate ini;
use ini::Ini;

extern crate scattering;
use scattering::structs::{Files, Bzone, Fields, Plot, Phonons};
use scattering::modelling::Model;

fn main() {
    let conf = Ini::load_from_file("config.ini").unwrap();
    let files = Files::from_config(&conf);
    let bzone = Bzone::from_config(&conf);
    let fields = Fields::from_config(&conf);
    let phonons = Phonons::from_config(&conf);
    let plot = Plot::from_config(&conf);
    let model = Model::from_config(&conf);
    let (energies, probabilities) = files.read_probabilities();

    files.clean_result();
    for f in plot.gen_fields(&fields) {
        let result = model.run(&bzone, &f, &phonons, &energies, &probabilities);
        files.append_result_line(&f, &result);
    }
}
