//! Provides function for calculate probability of electron-phonon scattering
use material::Material;
use linalg::Point;

/// Calculates $\int\limits\_{BZ} \delta(E(p)-E) d\^{2} p$
pub fn probability<T: Material>(energy: f64, m: &T, error: f64) -> f64 {
    use std::f64::consts::PI;
    use std::cmp::min;

    let mut old: f64 = 0.0;
    let mut new: f64 = 1.0;
    let mut n = 500;
    let mut iters_left = 12;
    while (new - old).abs() / new > error && iters_left > 0 {
        old = new;
        new = 0.0;

        let mut prev: Vec<Point> = Vec::new();

        for i in 0..n {
            let theta = (i as f64) / (n as f64) * 2.0 * PI;
            let curr = m.momentums(energy, theta);
            let l = min(curr.len(), prev.len());
            for j in 0..l {
                new += (curr[j] - prev[j]).len() / m.energy_gradient(&curr[j]).len();
            }
            prev = curr;
        }
        n *= 2;
        iters_left -= 1;
    }
    new
}
