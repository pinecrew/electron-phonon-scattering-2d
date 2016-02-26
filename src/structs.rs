struct Files {
    load_file: bool,
    probability: String,
    result: String
}

struct Phonos {
    beta: f64,
    wla_max: f64,
    wlo_max: f64,
    T: f64
}

struct Vec2 {
    x: f64,
    y: f64,
}

struct Point {
    x: f64,
    y: f64
}

struct Fields {
    // E = (E0, E1, E2)
    E: (Vec2, Vec2, Vec2),
    // B = (B0, B1, B2)
    B: (f64, f64, f64),
    // omega = (omega1, omega2)
    omega: (f64, f64),
    // phi = (phi, phi1, phi2)
    phi: (f64, f64, f64)
}

struct Bzone {
    A: Point,
    B: Point,
    C: Point,
    D: Point,
    basis: Vec2,
    dual_basis: Vec2
}

struct Probability {
    momentum_error: u64,
    probability_error: u64,
    probability: u64,
    energy: u64,
    momentum_samples: i32,
    energy_samples: i32
}

struct Model {
    dt: u64,
    all_time: u64,
    threads: u32,
    particles: u32
}

struct Plot {
    low: f64,
    hight: f64,
    step: f64,
    var: String
}

struct ExecuteModel {
    phonons: Phobos,
    bzone: Bzone,
    model: Model
}

impl Plot {
    fn gen_model(field: &Fields) -> Model {
        unimplemented!()
    }
}

impl ExecuteModel {
    fn new(phonons: Phobos, bzone: Bzone) -> ExecuteModel {
        ExecuteModel {
            phonons: phonons,
            bzone: bzone,
            model: Model::null(),
        }
    }
    fn set_model(&mut self, model: Model) {
        self.model = model;
    }
    fn run(&self) {
        unimplemented!()
    }
}
