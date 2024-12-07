pub struct Perceptron {
    pub weights: Vec<i32>,
    pub l: u32,
    pub n: u8,
}

pub enum LearningRule {
    Hebbian,
    AntiHebbian,
    RandomWalk,
}

impl Perceptron {
    pub fn new(l: u32, n: u8) -> Self {
        Perceptron { weights: vec![0;n.into()], l, n }
    }

    pub fn predict(&self, x: &[i32]) -> i32 {
        let mut sum = 0;
        for i in 0..self.n {
            sum += self.weights[i as usize] * x[i as usize];
        }
        Self::sgn(sum)
    }

    pub fn train(&mut self, x: &[i32], tau_a: i32, tau_b: i32, rule: LearningRule) {
        let pred = self.predict(x);
        for i in 0..self.n {
            let weight = &mut self.weights[i as usize];
            let x_i = x[i as usize];
            match rule {
                LearningRule::Hebbian => {
                    *weight =
                        *weight +
                        x_i * tau_a * Self::theta(pred * tau_b) * Self::theta(tau_a * tau_b);
                }
                LearningRule::AntiHebbian => {
                    *weight =
                        *weight -
                        x_i * tau_a * Self::theta(pred * tau_b) * Self::theta(tau_a * tau_b);
                }
                LearningRule::RandomWalk => {
                    *weight = *weight + x_i * Self::theta(pred * tau_b);
                }
            }
            if *weight > (self.l as i32) {
                *weight = (self.l as i32) * Self::sgn(*weight);
            }
        }
    }

    fn theta(a: i32) -> i32 {
        if a > 0 { 1 } else { 0 }
    }
    fn sgn(a: i32) -> i32 {
        if a > 0 { 1 } else { -1 }
    }
}
