use linalg::{Vec2, Point, Cross};

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
pub struct Bzone {
    pub A: Point,
    pub B: Point,
    pub C: Point,
    pub D: Point,
    pub basis: (Vec2, Vec2),
    pub dual_basis: (Vec2, Vec2),
}

impl Bzone {
    pub fn new(A: Point, B: Point, D: Point) -> Bzone {
        let b = B - A;
        let d = D - A;
        let C = A + b + d;

        let b2 = b - b.ort() * b.dot(d.ort());
        let d2 = d - d.ort() * d.dot(b.ort());

        let b1 = b2 / b.dot(b2);
        let d1 = d2 / d.dot(d2);

        Bzone {
            A: A,
            B: B,
            C: C,
            D: D,
            basis: (b, d),
            dual_basis: (b1, d1),
        }
    }

    // Функция, приводящая квазиимпульс к первой зоне Бриллюэна
    pub fn to_first_bz(&self, p: &Point) -> Point {

        let pv = *p - self.A;

        // находим разложение по базису, используя взаимный базис
        let n1 = pv.dot(self.dual_basis.0).floor();
        let n2 = pv.dot(self.dual_basis.1).floor();

        *p - self.basis.0 * n1 - self.basis.1 * n2
    }

    // Границы первой зоны Бриллюэна
    pub fn pmax(&self, theta: f64) -> f64 {
        // Считаем расстояние от начала координат до точки пересечения луча с
        // отрезками
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
