#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use linalg::{Vec2, Point, Cross};
use structs::Bzone;


const vf : f64 = 1e6;
const hbar : f64 = 6.5e-16;
const c : f64 = 3e8;
const d : f64 = 2e-8;
const eps0 : f64 = 0.059;
const eps1 : f64 = 0.029;
const g : f64 = (eps1 / eps0) * (eps1 / eps0);
const a : f64 = vf * hbar / eps0 / d;


/*
    *
    * Выражение для энергетического спектра (в декартовых координатах)
    *
*/
pub fn energy(p : &Point) -> f64 {
    let root = (1.0 + a * a * p.x * p.x).sqrt();
    eps0 * (root + g * (1.0 - p.y.cos()) / root)
}
/*
    *
    * Выражение для энергетического спектра (в полярных координатах)
    *
*/
pub fn energy_theta(p : f64, theta : f64) -> f64 {
    energy(&Point::from_polar(p, theta))
}

/*
    *
    * Градиент энергии в импульсном пространстве
    *
*/
pub fn energy_gradient(p : &Point) -> Vec2 {
    let b = 1.0 + a * a * p.x * p.x;
    let root = b.sqrt();
    Vec2::new(eps0 * a * a * p.x / root * (1.0 - g * (1.0 - p.x.cos()) / b),
            g * eps0 / root * p.y.sin())
}
/*
    *
    * Скорость
    *
*/
pub fn velocity(p : &Point) -> Vec2 {
    energy_gradient(&p) * (d / hbar / c)
}


/*
    *
    * Границы первой зоны Бриллюэна
    *
*/
pub fn pmax(theta : f64, bzone : &Bzone) -> f64 {
    // Считаем расстояние от начала координат до точки пересечения луча с
    // отрезками
    let OA = bzone.A.position();
    let OB = bzone.B.position();
    let OC = bzone.C.position();
    let OD = bzone.D.position();
    let l = Vec2::from_polar(1.0, theta);

    let mut res = -1.0;

    let vs = vec![OA, OB, OC, OD, OA];
    for i in 0..4 {
        let p = vs[i].cross(vs[i + 1]) / l.cross(vs[i + 1] - vs[i]);

        if p < 0.0 {
            continue;
        }

        if res < 0.0 || p < res {
            res = p;
        }
    }

    res
}

/*
    *
    * Функция, приводящая квазиимпульс к первой зоне Бриллюэна
    *
*/
pub fn to_first_bz(p : Point, bzone : Bzone) -> Point {

    let pv = p - bzone.A;

    // находим разложение по базису, используя взаимный базис
    let n1 = pv.dot(bzone.dual_basis.0).floor();
    let n2 = pv.dot(bzone.dual_basis.1).floor();

    p - bzone.basis.0 * n1 - bzone.basis.1 * n2
}

pub fn get_energy_limits(bzone : &Bzone) -> (f64, f64) {
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