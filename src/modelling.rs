use ini::Ini;
use structs::{Bzone, Fields, Plot, Phonons, Res};
use linalg::{Point, Vec2, Cross};
use material_specific::{velocity, energy, to_first_bz, momentums_with_energy_in_dir};

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
            y: 362_436_069,
            z: 521_288_629,
            w: 88_675_123
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

#[derive(Debug)]
struct ParticleStats {
    average_speed: Vec2,
    n_ac: u32,
    n_opt: u32,
    tau: f64
}

impl ParticleStats {
    pub fn new(average_speed : Vec2, n_ac : u32, n_opt : u32, tau : f64) -> ParticleStats {
        ParticleStats {
            average_speed: average_speed,
            n_ac: n_ac,
            n_opt: n_opt,
            tau: tau
        }
    }
}

fn runge<F>(p : &Point, force : F, t : f64, dt : f64) -> Point
    where F : Fn(&Point, f64) -> Vec2 {

    let k1 = force(p, t);
    let k2 = force(&(*p + k1 * dt / 2.0), t + dt / 2.0);
    let k3 = force(&(*p + k2 * dt / 2.0), t + dt / 2.0);
    let k4 = force(&(*p + k3 * dt), t + dt);

    *p + (k1 + k2 * 2.0 + k3 * 2.0 + k4) * dt / 6.0
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
        let section = conf.section(Some("model".to_owned())).unwrap();
        let dt: f64 = get_element!(section, "dt");
        let all_time: f64 = get_element!(section, "all_time");
        let threads: u32 = get_element!(section, "threads");
        let particles: u32 = get_element!(section, "particles");
        Model::new(dt, all_time, threads, particles)
    }

    pub fn run(&self, b : &Bzone, f : &Fields, ph : &Phonons, es : &Vec<f64>, ps : &Vec<f64>) -> Res {
        unimplemented!();
    }

    fn one_particle(&self, init_condition : Point, seed : u32, b : &Bzone, f : &Fields, ph : &Phonons, es : &Vec<f64>, ps : &Vec<f64>) -> ParticleStats {
        use std::f64::consts::PI;

        let mut rng = Rng::new(seed);
        let mut p = init_condition;

        let mut t = 0.0;
        let mut wsum : f64 = 0.0;

        let mut n_ac = 0;
        let mut n_opt = 0;
        let mut int_v_dt = Vec2::zero();

        let force = |p : &Point, t : f64| -> Vec2 {
            f.E.0 + f.E.1 * (f.omega.0 * t).cos() +
            f.E.2 * (f.omega.1 * t + f.phi).cos() +
            velocity(p).cross(
            f.B.0 +  f.B.1 * (f.omega.0 * t).cos() +
            f.B.2 * (f.omega.1 * t + f.phi).cos())
        };

        let mut r = -rng.uniform().ln();
        while (t < self.all_time) {
            let v = velocity(&p);

            int_v_dt = int_v_dt + v * self.dt;

            p = runge(&p, &force, t, self.dt); // решаем уравнения движения

            /* приводим импульс к зоне */
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
