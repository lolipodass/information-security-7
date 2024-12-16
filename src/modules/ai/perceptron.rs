use std::fmt::Display;

use rand::Rng;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Perceptron {
    pub weights: Vec<i32>,
    pub l: u32,
    pub n: u8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LearningRule {
    Hebbian,
    AntiHebbian,
    RandomWalk,
}

impl Perceptron {
    pub fn new(weight_range: u32, inputs_amount: u8) -> Self {
        let mut rand = rand::thread_rng();

        let weights = (0..inputs_amount)
            .map(|_| rand.gen_range(-(weight_range as i32)..=weight_range as i32))
            .collect();
        Perceptron { weights: weights, l: weight_range, n: inputs_amount }
    }

    pub fn predict(&self, x: &[i32]) -> i32 {
        let mut sum = 0;
        for i in 0..self.n {
            sum += self.weights[i as usize] * x[i as usize];
        }
        Self::sgn(sum)
    }

    pub fn train(&mut self, x: &[i32], tau: i32, rule: LearningRule) {
        let pred = self.predict(x);
        if pred != tau {
            return;
        }
        for i in 0..self.n {
            let weight = &mut self.weights[i as usize];
            let x_i = x[i as usize];
            *weight = match rule {
                LearningRule::Hebbian => x_i * pred,
                LearningRule::AntiHebbian => -x_i * pred,
                LearningRule::RandomWalk => x_i,
            };
            if *weight > (self.l as i32) {
                *weight = (self.l as i32) * Self::sgn(*weight);
            }
            if *weight < -(self.l as i32) {
                *weight = -(self.l as i32) * Self::sgn(*weight);
            }
        }
    }
    fn sgn(a: i32) -> i32 {
        if a > 0 { 1 } else { -1 }
    }
}

impl Display for Perceptron {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " Weights: {:2?}", self.weights)
    }
}

#[test]
fn test_perceptron() {
    let perceptron = Perceptron::new(1, 2);
    println!("perceptron: {:#?}", perceptron);
    println!("perceptron: {}", perceptron);
}
