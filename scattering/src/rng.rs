/// Xorshift128 pseudo-random number generator
pub struct Rng {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl Rng {
    pub fn new(seed: u32) -> Rng {
        Rng {
            x: seed,
            y: 362_436_069,
            z: 521_288_629,
            w: 88_675_123,
        }
    }
    pub fn rand(&mut self) -> u32 {
        let t = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = self.w ^ (self.w >> 19) ^ t ^ (t >> 8);
        self.w
    }
    pub fn uniform(&mut self) -> f64 {
        self.rand() as f64 / u32::max_value() as f64
    }
}

#[test]
fn test_uniform() {
    let mut rng = Rng::new(12345);
    let samples: usize = 1_000_000;
    let intervals: usize = 100;
    let width: f64 = 1.0 / intervals as f64;
    let mut distrib: Vec<usize> = vec![0; intervals];
    for _ in 0..samples {
        let r = rng.uniform();
        let p = (r / width).floor() as usize;
        distrib[p] += 1;
    }
    for i in 0..intervals {
        let density = distrib[i] as f64 / samples as f64 / width;
        let stdev = ((1.0 - width) / samples as f64 / width).sqrt();
        assert!((density - 1.0).abs() < 3.0 * stdev);
    }
}
