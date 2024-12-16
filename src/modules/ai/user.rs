use rand::Rng;

use crate::modules::ai::{ neural_network::NeuralNetwork, perceptron::LearningRule };

pub fn usage(
    range: u32,
    input_amount: u8,
    perceptron_amount: u32,
    rule: LearningRule
) -> (NeuralNetwork, NeuralNetwork, u32) {
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
        if rounds % 10000 == 0 {
            break;
        }
    }

    (user_a, user_b, rounds)
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
    use std::collections::HashMap;

    let range = 4;
    let input_amount = 8;
    let perceptron_amount = 4;
    let rule = LearningRule::RandomWalk;

    println!("range(L): {}", range);
    println!("input_amount(N): {}", input_amount);
    println!("perceptron_amount(K): {}", perceptron_amount);

    let mut hash_map = HashMap::new();

    for _ in 0..10000 {
        let (user_a, user_b, rounds) = usage(range, input_amount, perceptron_amount, rule);
        println!("Rounds: {}", rounds);
        println!("user_a:");
        user_a.show_matrix();
        println!("user_b:");
        user_b.show_matrix();
        println!("user_a == user_b: {}", user_a == user_b);

        if user_a == user_b {
            let count = hash_map.entry(rounds).or_insert(0);
            *count += 1;
        }
    }

    let mut results: Vec<_> = hash_map.iter().collect();
    results.sort_by(|a, b| a.0.cmp(b.0));

    // let mut content = format!("Round\tAmount\n");

    // for (key, value) in results.clone() {
    //     let val = match key {
    //         _ => &key.to_string(),
    //     };
    //     content.push_str(&format!("{}\t{}\n", val, value));
    // }
    // let _ = fs::write("test.csv", content);

    println!("{:?}", results);
}
