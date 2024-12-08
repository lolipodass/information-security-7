use rand::Rng;

use crate::modules::ai::{ neural_network::NeuralNetwork, perceptron::LearningRule };

pub fn usage() {
    let range = 4;
    let input_amount = 8;
    let perceptron_amount = 4;
    let rule = LearningRule::RandomWalk;

    println!("range(L): {}", range);
    println!("input_amount(N): {}", input_amount);
    println!("perceptron_amount(K): {}", perceptron_amount);

    let mut user_a = NeuralNetwork::new(range, input_amount, perceptron_amount, rule);
    let mut user_b = NeuralNetwork::new(range, input_amount, perceptron_amount, rule);

    let mut rounds = 0;
    while user_a != user_b {
        let block = generate_block(input_amount);
        let user_a_pred = user_a.predict(&block);
        let user_b_pred = user_b.predict(&block);

        if user_a_pred == user_b_pred {
            user_a.train(&block, user_b_pred);
            user_b.train(&block, user_a_pred);
        }

        rounds += 1;
        if rounds % 5000 == 0 {
            break;
        }
    }
    println!("Rounds: {}", rounds);
    println!("user_a: {}", user_a);
    println!("user_b: {}", user_b);
    println!("user_a == user_b: {}", user_a == user_b);
}

fn generate_block(l: u8) -> Vec<i32> {
    let mut block = Vec::with_capacity(l as usize);
    let mut rand = rand::thread_rng();
    for _ in 0..l {
        let val = rand.gen_bool(0.5);
        if val {
            block.push(1);
        } else {
            block.push(-1);
        }
    }
    block
}

#[test]
fn test_usage() {
    usage();
}
