//! Modelling one particle movement in material under electromagnetic fields with phonon scattering

use material::Material;
use phonons::Phonons;
use linalg::{Point, Vec2, Cross};
use rng::Rng;

#[derive(Clone)]
/// Electromagnetic fields
pub struct Fields {
    /// Amplitudes of constant, first and second wave electric fields
    pub e: (Vec2, Vec2, Vec2),
    /// Amplitudes of z-component constant, first and second wave magnetic fields
    pub b: (f64, f64, f64),
    /// Frequences of waves. First value is unused and exists for consistency
    pub omega: (f64, f64, f64),
    pub phi: f64,
}

impl Fields {
    pub fn new(e: (Vec2, Vec2, Vec2), b: (f64, f64, f64), omega: (f64, f64), phi: f64) -> Fields {
        Fields {
            e: e,
            b: b,
            omega: (0.0, omega.0, omega.1),
            phi: phi,
        }
    }
    pub fn zero() -> Fields {
        Fields::new((Vec2::zero(), Vec2::zero(), Vec2::zero()),
                    (0.0, 0.0, 0.0),
                    (0.0, 0.0),
                    0.0)
    }
}

fn runge<F>(p: &Point, force: F, t: f64, dt: f64) -> Point
    where F: Fn(&Point, f64) -> Vec2
{

    let k1 = force(p, t);
    let k2 = force(&(*p + k1 * dt / 2.0), t + dt / 2.0);
    let k3 = force(&(*p + k2 * dt / 2.0), t + dt / 2.0);
    let k4 = force(&(*p + k3 * dt), t + dt);

    *p + (k1 + k2 * 2.0 + k3 * 2.0 + k4) * dt / 6.0
}

#[derive(Clone)]
pub struct Summary {
    pub average_speed: Vec2,
    pub acoustic: u32,
    pub optical: u32,
    pub tau: f64,
}

impl Summary {
    pub fn new(v: Vec2, a: u32, o: u32, t: f64) -> Summary {
        Summary {
            average_speed: v,
            acoustic: a,
            optical: o,
            tau: t,
        }
    }
}

pub struct Modelling {
    dt: f64,
    all_time: f64,
    init_condition: Point,
    seed: u32,
}

impl Modelling {
    pub fn new(dt: f64, all_time: f64, init_condition: Point, seed: u32) -> Modelling {
        Modelling {
            dt: dt,
            all_time: all_time,
            init_condition: init_condition,
            seed: seed,
        }
    }

    pub fn run<T: Material>(&self, m: &T, f: &Fields, ph: &Phonons) -> Summary {
        use std::f64::consts::PI;

        let mut rng = Rng::new(self.seed);
        let mut p = self.init_condition;

        let mut t = 0.0;
        let mut wsum: f64 = 0.0;

        let mut n_ac = 0;
        let mut n_opt = 0;
        let mut int_v_dt = Vec2::zero();

        let force = |p: &Point, t: f64| -> Vec2 {
            f.e.0 + f.e.1 * (f.omega.1 * t).cos() + f.e.2 * (f.omega.2 * t + f.phi).cos() +
            m.velocity(p)
             .cross(f.b.0 + f.b.1 * (f.omega.1 * t).cos() + f.b.2 * (f.omega.2 * t + f.phi).cos())
        };

        let mut r = -rng.uniform().ln();
        while t < self.all_time {
            let v = m.velocity(&p);

            int_v_dt = int_v_dt + v * self.dt;

            p = runge(&p, &force, t, self.dt); // решаем уравнения движения

            // приводим импульс к зоне
            p = m.brillouin_zone().to_first_bz(&p);

            t += self.dt;

            let mut e = m.energy(&p);
            let dwlo = ph.optical_constant * ph.probability(e - ph.optical_energy); // 0, если выпал из минизоны
            let dwla = ph.acoustic_constant * ph.probability(e);
            wsum += (dwla + dwlo) * self.dt;

            if wsum > r {
                r = -rng.uniform().ln();
                wsum = 0.0;
                if dwlo / (dwla + dwlo) > rng.uniform() {
                    n_opt += 1; // наращиваем счетчик рассеяний на оптических
                             // фононах
                    e -= ph.optical_energy;
                } else {
                    n_ac += 1; // наращиваем счетчик рассеяний на акустических фононах
                }
                let mut count = 15;
                while count > 0 {
                    let theta = 2.0 * PI * rng.uniform(); // случайным образом
                    // разыгрываем направление квазиимпульса
                    let ps = m.momentums(e, theta);
                    if ps.len() > 0 {
                        p = ps[0];
                        break;
                    }
                    // если p существует, то мы правильно
                    // подобрали угол рассеяния, поэтому выходим из цикла
                    // если за 15 попыток не нашли решение, выходим из цикла
                    count -= 1;
                }
            }
        }
        let n0 = n_ac + n_opt;
        let average_speed = int_v_dt / t;
        let tau = t / (n0 as f64 + 1.0);

        Summary {
            average_speed: average_speed,
            acoustic: n_ac,
            optical: n_opt,
            tau: tau,
        }
    }
}