pub struct Phonons {
    pub optical_energy: f64,
    pub acoustic_constant: f64,
    pub optical_constant: f64,
    energies: Vec<f64>,
    probabilities: Vec<f64>,
}

impl Phonons {
    pub fn new(optical_energy: f64,
               optical_constant: f64,
               acoustic_constant: f64,
               energies: Vec<f64>,
               probabilities: Vec<f64>)
               -> Phonons {
        Phonons {
            optical_energy: optical_energy,
            acoustic_constant: acoustic_constant,
            optical_constant: optical_constant,
            energies: energies,
            probabilities: probabilities,
        }
    }

    pub fn probability(&self, energy: f64) -> f64 {
        let step = self.energies[1] - self.energies[0];

        if energy < self.energies[0] || energy > self.energies[self.energies.len() - 1] {
            return 0.0;
        }

        let pos = (energy - self.energies[0]) / step;
        let i = pos.floor() as usize;
        let w = pos - pos.floor();
        self.probabilities[i] * (1.0 - w) + self.probabilities[i + 1] * w
    }
}


#[test]
fn test_probability() {
    let ph = Phonons::new(0.0, 0.0, 0.0, vec![0.0, 0.5, 1.0], vec![1.0, 2.0, 2.0]);
    assert_eq!(ph.probability(0.25), 1.5);
    assert_eq!(ph.probability(0.75), 2.0);
    assert_eq!(ph.probability(-1.0), 0.0);
    assert_eq!(ph.probability(1.1), 0.0);
}
