extern crate scattering;
use scattering::material::{Material, BrillouinZone};

const vf: f64 = 1e6;
const hbar: f64 = 6.5e-16;
const c: f64 = 3e8;
const d: f64 = 2e-8;
const eps0: f64 = 0.059;
const eps1: f64 = 0.029;
const g: f64 = (eps1 / eps0) * (eps1 / eps0);
const a: f64 = vf * hbar / eps0 / d;


#[derive(Debug)]
pub struct SL {
    minimum_energy: f64,
    maximum_energy: f64,
    brillouin_zone: BrillouinZone,
}

impl SL {
    fn new() -> SL {
        unimplemented!();
    }

    fn get_energy_limits(bzone: &Bzone) -> (f64, f64) {
        let mut emin = 100.0;
        let mut emax = -100.0;
        let n = 1000;
        for i in 0..n {
            for j in 0..n {
                let i1 = (i as f64) / (n as f64);
                let j1 = (j as f64) / (n as f64);
                let p = bzone.A + bzone.basis.0 * i1 + bzone.basis.1 * j1;
                let e = energy(&p);
                if e < emin {
                    emin = e;
                }
                if e > emax {
                    emax = e;
                }
            }
        }
        (emin, emax)
    }
}

impl Material for SL {
    // Выражение для энергетического спектра (в декартовых координатах)
    pub fn energy(p: &Point) -> f64 {
        let root = (1.0 + a * a * p.x * p.x).sqrt();
        eps0 * (root + g * (1.0 - p.y.cos()) / root)
    }


    // Градиент энергии в импульсном пространстве
    pub fn energy_gradient(p: &Point) -> Vec2 {
        let b = 1.0 + a * a * p.x * p.x;
        let root = b.sqrt();
        Vec2::new(eps0 * a * a * p.x / root * (1.0 - g * (1.0 - p.x.cos()) / b),
                  g * eps0 / root * p.y.sin())
    }

    // Скорость
    pub fn velocity(p: &Point) -> Vec2 {
        energy_gradient(&p) * (d / hbar / c)
    }





    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn momentums_with_energy_in_dir(e: f64, theta: f64, samples: usize,
                                    precision: f64, bzone: &Bzone) -> Vec<Point> {
        let dir = Vec2::from_polar(1.0, theta);
        let step = dir * pmax(theta, bzone) / (samples as f64);

        let mut ps: Vec<Point> = Vec::new();

        for i in 0..samples {
            let mut left = Point::from_vec2(step * i as f64);
            let mut right = left + step;
            if (energy(&left) - e) * (energy(&right) - e) < 0.0 {
                while (right - left).len() > precision {
                    let middle = left + (right - left) / 2.0;
                    if (energy(&left) - e) * (energy(&middle) - e) < 0.0 {
                        right = middle;
                    } else {
                        left = middle;
                    }
                }

                ps.push(left + (right - left) / 2.0);
            }
        }
        ps
    }
}
