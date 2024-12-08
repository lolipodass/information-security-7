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

    pub fn train(&mut self, x: &[i32], tau_a: i32, tau_b: i32, rule: LearningRule) {
        let pred = self.predict(x);
        for i in 0..self.n {
            let weight = &mut self.weights[i as usize];
            let x_i = x[i as usize];
            match rule {
                LearningRule::Hebbian => {
                    *weight =
                        *weight + pred * x_i * Self::theta(pred, tau_a) * Self::theta(tau_a, tau_b);
                }
                LearningRule::AntiHebbian => {
                    *weight =
                        *weight - pred * x_i * Self::theta(pred, tau_a) * Self::theta(tau_a, tau_b);
                }
                LearningRule::RandomWalk => {
                    *weight = *weight + x_i * Self::theta(pred, tau_a) * Self::theta(tau_a, tau_b);
                }
            }
            if *weight > (self.l as i32) {
                *weight = (self.l as i32) * Self::sgn(*weight);
            }
            if *weight < -(self.l as i32) {
                *weight = -(self.l as i32) * Self::sgn(*weight);
            }
        }
    }

    fn theta(a: i32, b: i32) -> i32 {
        (a == b) as i32
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

