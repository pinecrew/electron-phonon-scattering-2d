use linalg::Vec2;

pub struct EnsembleStats {
    pub current: Vec2,
    pub current_std: Vec2,
    pub optical: f64,
    pub acoustic: f64,
    pub tau: f64
}

impl EnsembleStats {
    pub fn new(current : Vec2, current_std : Vec2, optical : f64, acoustic : f64, tau : f64) -> EnsembleStats {
        EnsembleStats {
            current: current,
            current_std: current_std,
            optical: optical,
            acoustic: acoustic,
            tau: tau
        }
    }

    pub fn from_ensemble(ensemble : &[ParticleStats]) -> EnsembleStats {
        let average_speed : Vec<Vec2> = ensemble.iter().map(|x| x.average_speed).collect();
        EnsembleStats::new(mean_Vec2(&average_speed),
                           std_mean_Vec2(&average_speed),
                           mean_u32(ensemble.iter().map(|x| x.optical).collect::<Vec<u32>>().as_ref()),
                           mean_u32(ensemble.iter().map(|x| x.acoustic).collect::<Vec<u32>>().as_ref()),
                           mean_f64(ensemble.iter().map(|x| x.tau).collect::<Vec<f64>>().as_ref()))
    }
}

#[derive(Debug)]
pub struct ParticleStats {
    average_speed: Vec2,
    optical: u32,
    acoustic: u32,
    tau: f64
}

impl ParticleStats {
    pub fn new(average_speed : Vec2, optical : u32, acoustic : u32, tau : f64) -> ParticleStats {
        ParticleStats {
            average_speed: average_speed,
            optical: optical,
            acoustic: acoustic,
            tau: tau
        }
    }
}

// переделать на дженериках и трейтах
pub fn mean_Vec2(data : &[Vec2]) -> Vec2 {
    let sum = data.iter().fold(Vec2::zero(), |acc, &x| acc + x);
    sum / data.len() as f64
}

pub fn mean_u32(data : &[u32]) -> f64 {
    let sum = data.iter().fold(0u32, |acc, &x| acc + x);
    sum as f64 / data.len() as f64
}

pub fn mean_f64(data : &[f64]) -> f64 {
    let sum = data.iter().fold(0.0, |acc, &x| acc + x);
    sum / data.len() as f64
}

#[test]
fn test_mean() {
    let data = vec![-2.0, -1.0, 0.0, 1.0, 2.0];
    assert_eq!(mean_f64(&data), 0.0);
    let data_u32 = vec![0, 1, 2, 3, 4, 5];
    assert_eq!(mean_u32(&data_u32), 2.5);
    let data_Vec2 = vec![Vec2::new(1.0, 2.0), Vec2::new(-3.0, 4.0)];
    assert_eq!(mean_Vec2(&data_Vec2), Vec2::new(-1.0, 3.0));
}

pub fn std_mean_Vec2(data : &[Vec2]) -> Vec2 {
    let mean = mean_Vec2(data);
    let sum = data.iter().fold(Vec2::zero(), |acc, &x| acc + (x - mean).sqr());
    sum.sqrt() / data.len() as f64
}

#[test]
fn test_std_mean() {
    let data_Vec2 = vec![Vec2::new(1.0, 2.0), Vec2::new(-3.0, 4.0)];
    assert!((std_mean_Vec2(&data_Vec2) - Vec2::new(2.0, 1.0) / f64::sqrt(2.0)).len() < 1e-10);
}