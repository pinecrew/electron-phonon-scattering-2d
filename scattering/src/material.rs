use linalg::{Vec2, Point, Cross, dual_basis};

pub trait Material {
    /// Energy spectrum of electrons
    fn energy(&self, p: &Point) -> f64;
    /// Energy spectrum in polar coordinates
    fn energy_polar(&self, p: f64, theta: f64) -> f64 {
        self.energy(&Point::from_polar(p, theta))
    }
    /// Gradient of energy in momentum space
    fn energy_gradient(&self, p: &Point) -> Vec2;

    fn velocity(&self, p: &Point) -> Vec2;
    /// Minimum of energy in brillouin zone
    fn min_energy(&self) -> f64;
    /// Maximum of energy in brillouin zone
    fn max_energy(&self) -> f64;
    /// Solves equation energy_polar(p, theta) = energy
    fn momentums(&self, energy: f64, theta: f64) -> Vec<Point>;
    /// brillouin zone structure
    fn brillouin_zone(&self) -> &BrillouinZone;
    /// optical phonon energy
    fn optical_energy(&self) -> f64;
    /// optical phonon scattering probability
    fn optical_scattering(&self, p: &Point) -> f64;
    /// acoustic phonon scattering probability
    fn acoustic_scattering(&self, p: &Point) -> f64;
}

#[derive(Clone)]
/// Representation of tetragonal brillouin zone of 2D material.
/// a, b, c, and d are vertices of tetragon.
pub struct BrillouinZone {
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub d: Point,
    pub basis: (Vec2, Vec2),
    pub dual_basis: (Vec2, Vec2),
}

impl BrillouinZone {
    pub fn new(a: Point, b: Point, d: Point) -> BrillouinZone {
        let v1 = b - a;
        let v2 = d - a;
        let c = a + v1 + v2;
        let basis = (v1, v2);

        BrillouinZone {
            a: a,
            b: b,
            c: c,
            d: d,
            basis: basis,
            dual_basis: dual_basis(basis),
        }
    }

    /// Returns equivalent momentum in first brillouin zone
    pub fn to_first_bz(&self, p: &Point) -> Point {

        let pv = *p - self.a;

        let n1 = pv.dot(self.dual_basis.0).floor();
        let n2 = pv.dot(self.dual_basis.1).floor();

        *p - self.basis.0 * n1 - self.basis.1 * n2
    }

    /// Calculates maximum value of momentum in direction $\theta$ in first brillouin zone
    pub fn pmax(&self, theta: f64) -> f64 {

        let oa = self.a.position();
        let ob = self.b.position();
        let oc = self.c.position();
        let od = self.d.position();
        let l = Vec2::from_polar(1.0, theta);

        let mut res = -1.0;

        let vs = vec![oa, ob, oc, od, oa];
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

macro_rules! assert_delta {
    ($x:expr, $y:expr, $d:expr) => {
        if !($x - $y < $d && $y - $x < $d) { panic!(); }
    }
}

#[test]
fn test_pmax() {
    let mut bz = BrillouinZone::new(Point::new(-4.0, -3.0),
                                    Point::new(4.0, -3.0),
                                    Point::new(-4.0, 3.0));
    assert_delta!(bz.pmax(0.0), 4.0, 1e-10);
    assert_delta!(bz.pmax((0.75f64).atan()), 5.0, 1e-10);
    assert_delta!(bz.pmax((0.5f64).atan()), 20f64.sqrt(), 1e-10);
    assert_delta!(bz.pmax((-0.75f64).atan()), 5.0, 1e-10);

    bz = BrillouinZone::new(Point::new(-4.0, -3.0),
                            Point::new(0.0, -3.0),
                            Point::new(0.0, 3.0));
    assert_delta!(bz.pmax(0.0), 2.0, 1e-10);
    assert_delta!(bz.pmax((0.75f64).atan()), 5.0, 1e-10);
    assert_delta!(bz.pmax((0.5f64).atan()), 11.25f64.sqrt(), 1e-10);
    assert_delta!(bz.pmax((-1.5f64).atan()), 3.25f64.sqrt(), 1e-10);
}

#[test]
fn test_to_first_bz() {
    let mut bz = BrillouinZone::new(Point::new(-4.0, -3.0),
                                    Point::new(4.0, -3.0),
                                    Point::new(-4.0, 3.0));
    assert_eq!(bz.to_first_bz(&Point::new(5.0, 3.0)),
               Point::new(-3.0, -3.0));
    assert!((bz.to_first_bz(&Point::new(15.3, -23.7)) - Point::new(-0.7, 0.3)).len() < 1e-10);

    bz = BrillouinZone::new(Point::new(-4.0, -3.0),
                            Point::new(0.0, -3.0),
                            Point::new(0.0, 3.0));
    assert_eq!(bz.to_first_bz(&Point::new(5.0, 3.0)),
               Point::new(-3.0, -3.0));
    assert!((bz.to_first_bz(&Point::new(15.3, -23.7)) - Point::new(-0.7, 0.3)).len() < 1e-10);
}