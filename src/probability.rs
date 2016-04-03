//! Provides function for calculate probability of electron-phonon scattering
use material::Material;
use linal::Point;

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
        let dtheta = 2.0 * PI / (n as f64);

        let mut prev: Vec<Point> = m.momentums(energy, 0f64);

        for i in 1..n + 1 {
            let theta = (i as f64) * dtheta;
            let curr = m.momentums(energy, theta);
            let l = min(curr.len(), prev.len());
            for j in 0..l {
                new += (curr[j] - prev[j]).len() / m.energy_gradient(&curr[j]).len();
            }

            // find endpoints
            for i in prev.len()..curr.len() {
                let mut dtheta = dtheta;
                let mut theta = theta;
                while dtheta > 1e-9 {
                    if m.momentums(energy, theta - dtheta).len() > i {
                        theta -= dtheta;
                    } else {
                        dtheta /= 2.0;
                    }
                }
                new += (curr[i] - m.momentums(energy, theta)[i]).len() /
                       m.energy_gradient(&curr[i]).len();
            }


            for i in curr.len()..prev.len() {
                let mut dtheta = dtheta;
                let mut theta = theta - dtheta;
                while dtheta > 1e-9 {
                    if m.momentums(energy, theta + dtheta).len() > i {
                        theta += dtheta;
                    } else {
                        dtheta /= 2.0;
                    }
                }
                new += (prev[i] - m.momentums(energy, theta)[i]).len() /
                       m.energy_gradient(&prev[i]).len();
            }

            prev = curr;
        }
        n *= 2;
        iters_left -= 1;
    }
    new
}
