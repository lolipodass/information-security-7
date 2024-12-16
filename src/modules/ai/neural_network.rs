use super::perceptron::{ LearningRule, Perceptron };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NeuralNetwork {
    layer: Vec<Perceptron>,
    pub weight_range: u32,
    pub perceptron_amount: u32,
    pub weight_amount: u8,
    pub rule: LearningRule,
}

impl NeuralNetwork {
    pub fn new(
        weight_range: u32,
        weight_amount: u8,
        perceptron_amount: u32,
        rule: LearningRule
    ) -> Self {
        let mut layer = Vec::with_capacity(perceptron_amount as usize);
        for _ in 0..perceptron_amount {
            layer.push(Perceptron::new(weight_range, weight_amount));
        }
        NeuralNetwork { layer, weight_range, weight_amount, perceptron_amount, rule }
    }

    pub fn predict(&self, x: &[i32]) -> i32 {
        let mut production = 1;
        for perceptron in self.layer.iter() {
            production *= perceptron.predict(x);
        }

        production
    }

    pub fn train(&mut self, x: &[i32], tau_b: i32) {
        let tau_a = self.predict(x);
        if tau_a != tau_b {
            return;
        }

        for perceptron in self.layer.iter_mut() {
            perceptron.train(x, tau_a, self.rule);
        }
    }
    pub fn show_matrix(&self) {
        for perceptron in &self.layer {
            println!("{:?}", perceptron.weights);
        }
    }
}

impl std::fmt::Display for NeuralNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for perceptron in &self.layer {
            write!(f, "{}", perceptron)?;
        }
        Ok(())
    }
}
