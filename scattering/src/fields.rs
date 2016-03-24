use linal::Vec2;

#[derive(Clone)]
/// Electromagnetic fields
pub struct Fields {
    /// Amplitudes of constant, first and second wave electric fields
    pub e: (Vec2, Vec2, Vec2),
    /// Amplitudes of z-component constant, first and second wave magnetic fields
    pub b: (f64, f64, f64),
    /// Frequences of waves. First value is unused and exists for consistency
    pub omega: (f64, f64, f64),
    pub phi: f64,
}

impl Fields {
    pub fn new(e: (Vec2, Vec2, Vec2), b: (f64, f64, f64), omega: (f64, f64), phi: f64) -> Fields {
        Fields {
            e: e,
            b: b,
            omega: (0.0, omega.0, omega.1),
            phi: phi,
        }
    }
    pub fn zero() -> Fields {
        Fields::new((Vec2::zero(), Vec2::zero(), Vec2::zero()),
                    (0.0, 0.0, 0.0),
                    (0.0, 0.0),
                    0.0)
    }
}
