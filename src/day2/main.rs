use std::fs;
use std::collections::HashMap;

fn main() {
    let mut opponent_indexes = HashMap::new();
    opponent_indexes.insert("A".to_string(), 1); // Rock
    opponent_indexes.insert("B".to_string(), 2); // Paper
    opponent_indexes.insert("C".to_string(), 3); // Scissors
    let mut score_values = HashMap::new();
    score_values.insert("X".to_string(), [1, 3, 0, 6]); // Rock
    score_values.insert("Y".to_string(), [2, 6, 3, 0]); // Paper
    score_values.insert("Z".to_string(), [3, 0, 6, 3]); // Scissors

    let file_path: &str = "src/day2/input.txt";
    let content: String = fs::read_to_string(file_path)
        .expect("File failed to read");
    let rounds: Vec<&str> = content.split('\n').collect();
    let mut score: usize = 0;

    for round in rounds.iter() {
        if round.len() != 3 {
            continue;
        }
        let opponent: String = String::from(round.chars().nth(0).unwrap());
        let me: String = String::from(round.chars().nth(2).unwrap());
        score += score_values[&me][0];
        score += score_values[&me][opponent_indexes[&opponent]];
        //println!("{opponent} vs {me}");
    }
    println!("Total score is: {score}");
}
