use linalg::{Vec2, Point, Cross, dual_basis};

pub trait Material {
    fn energy(p: &Point) -> f64;

    fn energy_polar(p: f64, theta: f64) -> f64 {
        self.energy(Vec2::from_polar(p, theta))
    }

    fn energy_gradient(p: &Point) -> Vec2;

    fn velocity(p: &Point) -> Vec2;

    fn min_energy() -> f64;

    fn max_energy() -> f64;

    fn momentums(energy: f64, theta: f64) -> Vec<Point>;

    fn brillouin_zone() -> BrillouinZone;
}

#[derive(Clone)]
pub struct BrillouinZone {
    pub A: Point,
    pub B: Point,
    pub C: Point,
    pub D: Point,
    pub basis: (Vec2, Vec2),
    pub dual_basis: (Vec2, Vec2),
}

impl BrillouinZone {
    pub fn new(A: Point, B: Point, D: Point) -> Bzone {
        let b = B - A;
        let d = D - A;
        let C = A + b + d;
        let basis = (b, d);

        Bzone {
            A: A,
            B: B,
            C: C,
            D: D,
            basis: basis,
            dual_basis: dual_basis(basis),
        }
    }

    /// Returns equivalent momentum in first brillouin zone
    pub fn to_first_bz(&self, p: &Point) -> Point {

        let pv = *p - self.A;

        let n1 = pv.dot(self.dual_basis.0).floor();
        let n2 = pv.dot(self.dual_basis.1).floor();

        *p - self.basis.0 * n1 - self.basis.1 * n2
    }

    /// Calculates maximum value of momentum in direction $\theta$ in first brillouin zone
    pub fn pmax(&self, theta: f64) -> f64 {

        let OA = self.A.position();
        let OB = self.B.position();
        let OC = self.C.position();
        let OD = self.D.position();
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
}
