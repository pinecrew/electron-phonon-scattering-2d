use ini::Ini;
use structs::{Bzone, Fields, Plot, Phonons, Res};
use linalg::{Point, Vec2};

struct Rng {
    x: u32,
    y: u32,
    z: u32,
    w: u32
}

impl Rng {
    pub fn new(seed: u32) -> Rng {
        Rng {
            x: seed,
            y: 2_123_435_612,
            z: 74_563_921,
            w: 912_234_638
        }
    }
    pub fn rand(&mut self) -> u32 {
        let t = (self.x ^ (self.x << 11));
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = (self.w ^ (self.w >> 19)) ^ (t ^ (t >> 8));
        self.w
    }
    pub fn uniform(&mut self) -> f64 {
        self.rand() as f64 / u32::max_value() as f64
    }
}

fn runge<F>(p : Point, force : F, t : f64, dt : f64) -> Point
    where F : Fn(Point, f64) -> Vec2 {

    let k1 = force(p, t);
    let k2 = force(p + k1 * dt / 2.0, t + dt / 2.0);
    let k3 = force(p + k2 * dt / 2.0, t + dt / 2.0);
    let k4 = force(p + k3 * dt, t + dt);

    p + (k1 + k2 * 2.0 + k3 * 2.0 + k4) * dt / 6.0
}

fn get_probability(e : f64, es : &Vec<f64>, ps : &Vec<f64>) -> f64 {
    let step = es[1] - es[0];
    let pos = (e - es[0]) / step;
    if pos < 0.0 || pos + 1.0 > ps.len() as f64 {
        return 0.0;
    }
    let i = pos.floor() as usize;
    let w = pos - pos.floor();
    ps[i] * (1.0 - w) + ps[i+1] * w
}

#[test]
fn test_probability() {
    let es = vec![0.0,0.5,1.0];
    let ps = vec![1.0,2.0,2.0];
    assert_eq!(get_probability(0.25,&es,&ps), 1.5);
    assert_eq!(get_probability(0.75,&es,&ps), 2.0);
    assert_eq!(get_probability(-1.0,&es,&ps), 0.0);
    assert_eq!(get_probability(1.1,&es,&ps), 0.0);
}

pub struct Model {
    pub dt: f64,
    pub all_time: f64,
    pub threads: u32,
    pub particles: u32,
}

impl Model {
    pub fn new(dt: f64, all_time: f64, threads: u32, particles: u32) -> Model {
        Model {
            dt: dt,
            all_time: all_time,
            threads: threads,
            particles: particles,
        }
    }
    pub fn from_config(conf : &Ini) -> Model {
        unimplemented!();
    }
    pub fn run(&self, b : &Bzone, f : &Fields, ph : &Phonons, es : &Vec<f64>, ps : &Vec<f64>) -> Res {
        unimplemented!();
    }

    fn one_particle(&self, b : &Bzone, f : &Fields, ph : &Phonons, es : &Vec<f64>, ps : &Vec<f64>) -> Vec2 {
        unimplemented!();
    }
}