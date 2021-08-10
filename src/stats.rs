use linal::Vec2;
use particle::Summary;

pub struct Stats {
    pub current: Vec2,
    pub current_std: Vec2,
    pub optical: f64,
    pub acoustic: f64,
    pub tau: f64,
    pub energy: f64,
    pub from_theta_ac: Vec<usize>,
    pub to_theta_ac: Vec<usize>,
    pub from_theta_op: Vec<usize>,
    pub to_theta_op: Vec<usize>,
    pub field_phase_ac: Vec<usize>,
    pub field_phase_op: Vec<usize>,
}

impl Stats {
    pub fn from_ensemble(ensemble: &[Summary]) -> Stats {
        let average_speed: Vec<Vec2> = ensemble.iter().map(|x| x.average_speed).collect();
        Stats {
            current: average_speed.mean(),
            current_std: average_speed.mean_std(),
            optical: ensemble.iter().map(|x| x.optical).collect::<Vec<u32>>().mean(),
            acoustic: ensemble.iter().map(|x| x.acoustic).collect::<Vec<u32>>().mean(),
            tau: ensemble.iter().map(|x| x.tau).collect::<Vec<f64>>().mean(),
            energy: ensemble.iter().map(|x| x.energy).collect::<Vec<f64>>().mean(),
            from_theta_ac: ensemble.iter().map(|x| x.from_theta_ac.as_slice().to_vec()).reduce(|acc, x| acc.iter().zip(x.iter()).map(|(a, b)| a + b).collect::<Vec<usize>>()).unwrap(),
            to_theta_ac: ensemble.iter().map(|x| x.to_theta_ac.as_slice().to_vec()).reduce(|acc, x| acc.iter().zip(x.iter()).map(|(a, b)| a + b).collect::<Vec<usize>>()).unwrap(),
            from_theta_op: ensemble.iter().map(|x| x.from_theta_op.as_slice().to_vec()).reduce(|acc, x| acc.iter().zip(x.iter()).map(|(a, b)| a + b).collect::<Vec<usize>>()).unwrap(),
            to_theta_op: ensemble.iter().map(|x| x.to_theta_op.as_slice().to_vec()).reduce(|acc, x| acc.iter().zip(x.iter()).map(|(a, b)| a + b).collect::<Vec<usize>>()).unwrap(),
            field_phase_ac: ensemble.iter().map(|x| x.field_phase_ac.as_slice().to_vec()).reduce(|acc, x| acc.iter().zip(x.iter()).map(|(a, b)| a + b).collect::<Vec<usize>>()).unwrap(),
            field_phase_op: ensemble.iter().map(|x| x.field_phase_op.as_slice().to_vec()).reduce(|acc, x| acc.iter().zip(x.iter()).map(|(a, b)| a + b).collect::<Vec<usize>>()).unwrap(),
        }
    }
}

pub struct Histogram {
    pub min: f64,
    pub max: f64,
    pub n_bins: usize,
    pub bins: Vec<usize>,
    r_bin_width: f64,
}

impl Histogram {
    pub fn new(min: f64, max: f64, n_bins: usize) -> Histogram {
        Histogram {
            min: min,
            max: max,
            n_bins: n_bins,
            bins: vec![0; n_bins],
            r_bin_width: (n_bins as f64) / (max - min),
        }
    }

    pub fn add(&mut self, value: f64) {
        let bin = (value - self.min) * self.r_bin_width;
        self.bins[bin as usize] += 1;
    }
}

pub trait Mean {
    type Output;

    fn mean(&self) -> Self::Output;
}

impl Mean for [Vec2] {
    type Output = Vec2;

    fn mean(&self) -> Self::Output {
        let sum = self.iter().fold(Vec2::zero(), |acc, &x| acc + x);
        sum / self.len() as f64
    }
}

impl Mean for [u32] {
    type Output = f64;

    fn mean(&self) -> Self::Output {
        let sum = self.iter().fold(0, |acc, &x| acc + x);
        sum as f64 / self.len() as f64
    }
}

impl Mean for [f64] {
    type Output = f64;

    fn mean(&self) -> Self::Output {
        let sum = self.iter().fold(0.0, |acc, &x| acc + x);
        sum / self.len() as f64
    }
}


#[test]
fn test_mean() {
    let data_f64 = vec![-2.0, -1.0, 0.0, 1.0, 2.0];
    assert_eq!(data_f64.mean(), 0.0);
    let data_u32 = vec![0, 1, 2, 3, 4, 5];
    assert_eq!(data_u32.mean(), 2.5);
    let data_vec2 = vec![Vec2::new(1.0, 2.0), Vec2::new(-3.0, 4.0)];
    assert_eq!(data_vec2.mean(), Vec2::new(-1.0, 3.0));
}


pub trait MeanStd {
    type Output;

    fn mean_std(&self) -> Self::Output;
}

impl MeanStd for [Vec2] {
    type Output = Vec2;

    fn mean_std(&self) -> Self::Output {
        let mean = self.mean();
        let sum = self.iter().fold(Vec2::zero(), |acc, &x| acc + (x - mean).sqr());
        sum.sqrt() / self.len() as f64
    }
}

impl MeanStd for [f64] {
    type Output = f64;

    fn mean_std(&self) -> Self::Output {
        let mean = self.mean();
        let sum = self.iter().fold(0f64, |acc, &x| acc + (x - mean).powi(2));
        sum.sqrt() / self.len() as f64
    }
}

#[test]
fn test_mean_std() {
    let data_vec2 = vec![Vec2::new(1.0, 2.0), Vec2::new(-3.0, 4.0)];
    assert!((data_vec2.mean_std() - Vec2::new(2.0, 1.0) / f64::sqrt(2.0)).len() < 1e-10);
}
