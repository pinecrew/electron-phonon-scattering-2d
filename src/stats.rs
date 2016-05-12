use linal::Vec2;
use particle::Summary;

pub struct Stats {
    pub current: Vec2,
    pub current_std: Vec2,
    pub optical: f64,
    pub acoustic: f64,
    pub tau: f64,
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
        }
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
