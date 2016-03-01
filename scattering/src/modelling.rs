use structs::{Bzone, Fields, Plot, Phonons};
use linalg::{Point, Vec2, Cross};
use material_specific::{velocity, energy, to_first_bz, momentums_with_energy_in_dir, energy_theta,
                        pmax};
use time::get_time;

fn runge<F>(p: &Point, force: F, t: f64, dt: f64) -> Point
    where F: Fn(&Point, f64) -> Vec2
{

    let k1 = force(p, t);
    let k2 = force(&(*p + k1 * dt / 2.0), t + dt / 2.0);
    let k3 = force(&(*p + k2 * dt / 2.0), t + dt / 2.0);
    let k4 = force(&(*p + k3 * dt), t + dt);

    *p + (k1 + k2 * 2.0 + k3 * 2.0 + k4) * dt / 6.0
}

fn get_probability(e: f64, es: &Vec<f64>, ps: &Vec<f64>) -> f64 {
    let step = es[1] - es[0];
    let pos = (e - es[0]) / step;
    if pos < 0.0 || pos + 1.0 > ps.len() as f64 {
        return 0.0;
    }
    let i = pos.floor() as usize;
    let w = pos - pos.floor();
    ps[i] * (1.0 - w) + ps[i + 1] * w
}

#[test]
fn test_probability() {
    let es = vec![0.0, 0.5, 1.0];
    let ps = vec![1.0, 2.0, 2.0];
    assert_eq!(get_probability(0.25, &es, &ps), 1.5);
    assert_eq!(get_probability(0.75, &es, &ps), 2.0);
    assert_eq!(get_probability(-1.0, &es, &ps), 0.0);
    assert_eq!(get_probability(1.1, &es, &ps), 0.0);
}

struct BolzmannDistrib {
    T: f64,
    angle_distrib: Vec<f64>,
    b: Bzone,
}

impl BolzmannDistrib {
    pub fn new(T: f64, b: &Bzone) -> BolzmannDistrib {
        use std::f64::consts::PI;
        let n = 1000;
        let mut angle_distrib: Vec<f64> = vec![0.0; n];
        for i in 0..n - 1 {
            let theta = 2.0 * PI * i as f64 / n as f64;
            let pm = pmax(theta, &b);
            angle_distrib[i + 1] = angle_distrib[i];
            for j in 0..n {
                let p = pm * j as f64 / (n - 1) as f64;
                angle_distrib[i + 1] += (-energy_theta(p, theta) / T).exp() * pm / (n - 1) as f64;
            }
        }
        for i in 0..n {
            angle_distrib[i] /= angle_distrib[n - 1];
        }
        BolzmannDistrib {
            T: T,
            angle_distrib: angle_distrib,
            b: (*b).clone(),
        }
    }
    pub fn make_dist(&self, n: usize) -> Vec<Point> {
        use time::get_time;
        let mut rng = Rng::new(get_time().nsec as u32);
        let mut points : Vec<Point> = Vec::with_capacity(n);
        for i in 0..n {
            let theta = self.angle(rng.uniform());
            let p = self.momentum(theta, rng.uniform());
            points.push(p);
        }
        points
    }
    fn momentum(&self, theta: f64, r: f64) -> Point {
        let n = 1000;
        let mut dist = vec![0.0; n];
        let pm = pmax(theta, &self.b);
        for i in 0..n - 1 {
            let p = pm * i as f64 / (n - 1) as f64;
            dist[i + 1] = dist[i] + (-energy_theta(p, theta) / self.T).exp() * pm / (n - 1) as f64;
        }
        for i in 0..n {
            dist[i] /= dist[n - 1];
        }
        let mut i = 0;
        let mut j = dist.len() - 1;

        while j - i > 1 {
            let m = (i + j) / 2;
            if dist[m] > r {
                j = m;
            } else {
                i = m;
            }
        }
        let w = (r - dist[i]) / (dist[j] - dist[i]);
        let p = (i as f64 + w) / (n - 1) as f64 * pm;
        Point::from_polar(p, theta)
    }
    fn angle(&self, r: f64) -> f64 {
        use std::f64::consts::PI;
        let mut i = 0;
        let mut j = self.angle_distrib.len() - 1;

        while j - i > 1 {
            let m = (i + j) / 2;
            if self.angle_distrib[m] > r {
                j = m;
            } else {
                i = m;
            }
        }
        let w = (r - self.angle_distrib[i]) / (self.angle_distrib[j] - self.angle_distrib[i]);
        2.0 * PI * (i as f64 + w)
    }
}

pub struct Model {
    pub dt: f64,
    pub all_time: f64,
    pub threads: u32,
    pub particles: usize,
}

impl Model {
    pub fn new(dt: f64, all_time: f64, threads: u32, particles: usize) -> Model {
        Model {
            dt: dt,
            all_time: all_time,
            threads: threads,
            particles: particles,
        }
    }
    // pub fn from_config(conf: &Ini) -> Model {
    //     let section = conf.section(Some("model".to_owned())).unwrap();
    //     let dt: f64 = get_element!(section, "dt");
    //     let all_time: f64 = get_element!(section, "all_time");
    //     let threads: u32 = get_element!(section, "threads");
    //     let particles: usize = get_element!(section, "particles");
    //     Model::new(dt, all_time, threads, particles)
    // }

    // pub fn run(&self,
    //            b: &Bzone,
    //            f: &Fields,
    //            ph: &Phonons,
    //            es: &Vec<f64>,
    //            ps: &Vec<f64>)
    //            -> EnsembleStats {
    //     let bd = BolzmannDistrib::new(ph.T, &b);
    //     let init_condition = bd.make_dist(self.particles);
    //     let mut seed: Vec<u32> = vec![0u32; self.particles];

    //     let mut rng = Rng::new(get_time().nsec as u32);
    //     for j in 0..self.particles as usize {
    //         seed[j] = rng.rand();
    //     }

    //     let mut ensemble: Vec<ParticleStats> =
    //         vec![ParticleStats::new(Vec2::zero(), 0, 0, 0.0); self.particles as usize];
    //     let mut pool = Pool::new(self.threads as u32);

    //     pool.scoped(|scope| {
    //         for (index, item) in ensemble.iter_mut().enumerate() {
    //             let f = f.clone();
    //             let b = b.clone();
    //             let ph = ph.clone();
    //             let es = es.clone();
    //             let ph = ph.clone();
    //             let ic = init_condition[index];
    //             let s = seed[index];
    //             scope.execute(move || {
    //                 *item = self.one_particle(ic, s, &b, &f, &ph, &es, &ps);
    //             });
    //         }
    //     });

    //     EnsembleStats::from_ensemble(&ensemble)
    // }

    fn one_particle(&self,
                    init_condition: Point,
                    seed: u32,
                    b: &Bzone,
                    f: &Fields,
                    ph: &Phonons,
                    es: &Vec<f64>,
                    ps: &Vec<f64>)
                    -> ParticleStats {
        use std::f64::consts::PI;

        let mut rng = Rng::new(seed);
        let mut p = init_condition;

        let mut t = 0.0;
        let mut wsum: f64 = 0.0;

        let mut n_ac = 0;
        let mut n_opt = 0;
        let mut int_v_dt = Vec2::zero();

        let force = |p: &Point, t: f64| -> Vec2 {
            f.E.0 + f.E.1 * (f.omega.0 * t).cos() + f.E.2 * (f.omega.1 * t + f.phi).cos() +
            velocity(p).cross(f.B.0 + f.B.1 * (f.omega.0 * t).cos() +
                              f.B.2 * (f.omega.1 * t + f.phi).cos())
        };

        let mut r = -rng.uniform().ln();
        while (t < self.all_time) {
            let v = velocity(&p);

            int_v_dt = int_v_dt + v * self.dt;

            p = runge(&p, &force, t, self.dt); // решаем уравнения движения

            // приводим импульс к зоне
            p = to_first_bz(&p, b);

            t += self.dt;

            let mut e = energy(&p);
            let dwlo = ph.wlo_max * get_probability(e - ph.beta, &es, &ps); // 0, если выпал из минизоны
            let dwla = ph.wla_max * get_probability(e, &es, &ps);
            wsum += (dwla + dwlo) * self.dt;

            if (wsum > r) {
                r = -rng.uniform().ln();
                wsum = 0.0;
                if (dwlo / (dwla + dwlo) > rng.uniform()) {
                    n_opt += 1; // наращиваем счетчик рассеяний на оптических
                             // фононах
                    e -= ph.beta;
                } else {
                    n_ac += 1; // наращиваем счетчик рассеяний на акустических фононах
                }
                let mut count = 15;
                while (count > 0) {
                    let theta = 2.0 * PI * rng.uniform(); // случайным образом
                    // разыгрываем направление квазиимпульса
                    let ps = momentums_with_energy_in_dir(e, theta, 20, 1e-3, &b);
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

        ParticleStats::new(average_speed, n_ac, n_opt, tau)
    }
}
