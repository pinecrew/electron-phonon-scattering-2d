//! Particle one particle movement in material under electromagnetic fields with phonon scattering

use material::Material;
use fields::Fields;
use linal::Vec2;
use rng::Rng;
use stats::Histogram;

fn runge<F>(p: Vec2, force: F, t: f64, dt: f64) -> Vec2
    where F: Fn(Vec2, f64) -> Vec2
{

    let k1 = force(p, t);
    let k2 = force(p + k1 * dt / 2.0, t + dt / 2.0);
    let k3 = force(p + k2 * dt / 2.0, t + dt / 2.0);
    let k4 = force(p + k3 * dt, t + dt);

    p + (k1 + k2 * 2.0 + k3 * 2.0 + k4) * dt / 6.0
}

#[test]
fn runge_circle() {
    use std::f64::consts::PI;
    let f = |p: Vec2, _: f64| p.cross();
    let dt = 0.01;
    let mut p = Vec2::new(1.0, 0.0);
    let mut t = 0.0;
    while t < PI {
        p = runge(p, &f, t, dt);
        t += dt;
    }
    p = runge(p, &f, t, PI - t);
    assert!((p - Vec2::new(-1.0, 0.0)).len() < 1e-8);
}

#[test]
fn runge_parabola() {
    let f = |_: Vec2, t: f64| Vec2::new(0.0, t);
    let dt = 0.01;
    let mut p = Vec2::new(1.0, 0.0);
    let mut t = 0.0;
    while t < 1.0 {
        p = runge(p, &f, t, dt);
        t += dt;
    }
    p = runge(p, &f, t, 1.0 - t);
    assert!((p - Vec2::new(1.0, 0.5)).len() < 1e-8);
}

#[test]
fn runge_sin() {
    use std::f64::consts::PI;
    let f = |_: Vec2, t: f64| Vec2::new(0.0, t.sin());
    let dt = 0.01;
    let mut p = Vec2::new(1.0, 0.0);
    let mut t = 0.0;
    while t < PI {
        p = runge(p, &f, t, dt);
        t += dt;
    }
    p = runge(p, &f, t, PI - t);
    assert!((p - Vec2::new(1.0, 2.0)).len() < 1e-8);
}


#[derive(Clone)]
pub struct Summary {
    pub average_speed: Vec2,
    pub acoustic: u32,
    pub optical: u32,
    pub tau: f64,
    pub energy: f64,
    pub from_theta_ac: Vec<usize>,
    pub to_theta_ac: Vec<usize>,
    pub from_theta_op: Vec<usize>,
    pub to_theta_op: Vec<usize>,
    pub field_phase_ac: Vec<usize>,
    pub field_phase_op: Vec<usize>,
}

impl Summary {
    pub fn new(v: Vec2, a: u32, o: u32, t: f64, e: f64) -> Summary {
        Summary {
            average_speed: v,
            acoustic: a,
            optical: o,
            tau: t,
            energy: e,
            from_theta_ac: vec![],
            to_theta_ac: vec![],
            from_theta_op: vec![],
            to_theta_op: vec![],
            field_phase_ac: vec![],
            field_phase_op: vec![],
        }
    }
    pub fn empty() -> Summary {
        Summary {
            average_speed: Vec2::zero(),
            acoustic: 0,
            optical: 0,
            tau: 0.0,
            energy: 0.0,
            from_theta_ac: vec![],
            to_theta_ac: vec![],
            from_theta_op: vec![],
            to_theta_op: vec![],
            field_phase_ac: vec![],
            field_phase_op: vec![],
        }
    }
}

#[derive(Debug)]
enum Scattering {
    Acoustic,
    Optical,
}

pub struct Particle<'a, T: 'a + Material> {
    pub init_condition: Vec2,
    seed: u32,
    m: &'a T,
}

impl<'a, T: 'a + Material> Particle<'a, T> {
    pub fn new(m: &T, init_condition: Vec2, seed: u32) -> Particle<T> {
        Particle {
            m: m,
            init_condition: init_condition,
            seed: seed,
        }
    }

    pub fn run(&self, dt: f64, all_time: f64, f: &Fields) -> Summary {
        use std::f64::consts::PI;

        let mut rng = Rng::new(self.seed);
        let mut p = self.init_condition;

        let mut t = 0.0;
        let mut wsum: f64 = 0.0;

        let mut n_ac = 0;
        let mut n_opt = 0;
        let mut int_v_dt = Vec2::zero();
        let mut int_e_dt: f64 = 0.0;

        let n_bins = 256;
        let mut from_theta_ac = Histogram::new(0.0, 2.0 * PI, n_bins);
        let mut to_theta_ac = Histogram::new(0.0, 2.0 * PI, n_bins);
        let mut from_theta_op = Histogram::new(0.0, 2.0 * PI, n_bins);
        let mut to_theta_op = Histogram::new(0.0, 2.0 * PI, n_bins);
        let mut field_phase_ac = Histogram::new(0.0, 2.0 * PI, n_bins);
        let mut field_phase_op = Histogram::new(0.0, 2.0 * PI, n_bins);

        let force = |p: Vec2, t: f64| -> Vec2 {
            -(f.e.0 + f.e.1 * (f.omega.1 * t).cos() + f.e.2 * (f.omega.2 * t + f.phi).cos() +
              self.m.velocity(p).cross() *
              (f.b.0 + f.b.1 * (f.omega.1 * t).cos() + f.b.2 * (f.omega.2 * t + f.phi).cos()))
        };

        let mut r = -rng.uniform().ln();
        while t < all_time {
            let v = self.m.velocity(p);

            int_v_dt = int_v_dt + v * dt;

            p = runge(p, &force, t, dt); // решаем уравнения движения

            // приводим импульс к зоне
            p = self.m.brillouin_zone().to_first_bz(p);


            let mut e = self.m.energy(p);
            int_e_dt = int_e_dt + e * dt;
            let dwlo = self.m.optical_scattering(p); // 0, если выпал из минизоны
            let dwla = self.m.acoustic_scattering(p);
            wsum += (dwla + dwlo) * dt;

            if wsum > r {
                r = -rng.uniform().ln();
                wsum = 0.0;
                let mut kind = Scattering::Acoustic;
                if dwlo / (dwla + dwlo) > rng.uniform() {
                    n_opt += 1; // наращиваем счетчик рассеяний на оптических
                             // фононах
                    e -= self.m.optical_energy();
                    kind = Scattering::Optical
                } else {
                    n_ac += 1; // наращиваем счетчик рассеяний на акустических фононах
                }
                let mut count = 15;
                let theta = (p.y.atan2(p.x) + 2.0 * PI) % (2.0 * PI);
                while count > 0 {
                    let dtheta = 2.0 * PI * rng.uniform(); // случайным образом
                    // разыгрываем направление квазиимпульса
                    let new_theta = (theta + dtheta) % (2.0 * PI);
                    let phase = f.omega.1 * t % (2.0 * PI);
                    let ps = self.m.momentums(e, new_theta);
                    if ps.len() > 0 {
                        p = ps[0];
                        match kind {
                            Scattering::Acoustic => {
                                from_theta_ac.add(theta);
                                to_theta_ac.add(new_theta);
                                field_phase_ac.add(phase);
                            },
                            Scattering::Optical => {
                                from_theta_op.add(theta);
                                to_theta_op.add(new_theta);
                                field_phase_op.add(phase);
                            },
                        };
                        break;
                    }
                    // если p существует, то мы правильно
                    // подобрали угол рассеяния, поэтому выходим из цикла
                    // если за 15 попыток не нашли решение, выходим из цикла
                    count -= 1;
                }
            }

            t += dt;
        }
        let n0 = n_ac + n_opt;
        let average_speed = int_v_dt / t;
        let tau = t / (n0 as f64 + 1.0);
        let energy = int_e_dt / t;

        Summary {
            average_speed: average_speed,
            acoustic: n_ac,
            optical: n_opt,
            tau: tau,
            energy: energy,
            from_theta_ac: from_theta_ac.bins,
            to_theta_ac: to_theta_ac.bins,
            from_theta_op: from_theta_op.bins,
            to_theta_op: to_theta_op.bins,
            field_phase_ac: field_phase_ac.bins,
            field_phase_op: field_phase_op.bins,
        }
    }
}
