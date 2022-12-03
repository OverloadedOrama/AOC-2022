use std::fs;
use std::collections::HashMap;

fn find_pairs(opponent: char) -> [char; 2] {
    // Return an array of what it beats and from what it loses from
    if opponent == 'A' { // Rock
        ['C', 'B']
    }
    else if opponent == 'B' { // Paper
        ['A', 'C']
    }
    else { // Scissors
        ['B', 'A']
    }
}

fn main() {
    let mut opponent_indexes = HashMap::new();
    opponent_indexes.insert('A', 1); // Rock
    opponent_indexes.insert('B', 2); // Paper
    opponent_indexes.insert('C', 3); // Scissors
    let mut score_values = HashMap::new(); // For part one
    score_values.insert('X', [1, 3, 0, 6]); // Rock
    score_values.insert('Y', [2, 6, 3, 0]); // Paper
    score_values.insert('Z', [3, 0, 6, 3]); // Scissors

    let mut action_map = HashMap::new(); // For part two
    action_map.insert('X', 0); // Lose
    action_map.insert('Y', 3); // Draw
    action_map.insert('Z', 6); // Win

    let file_path: &str = "src/day2/input.txt";
    let content: String = fs::read_to_string(file_path)
        .expect("File failed to read");
    let rounds: Vec<&str> = content.split('\n').collect();
    let mut score_part1: usize = 0;
    let mut score_part2: usize = 0;

    for round in rounds.iter() {
        if round.len() != 3 {
            continue;
        }
        let opponent: char = round.chars().nth(0).unwrap();
        let me: char = round.chars().nth(2).unwrap();
        // Part 1
        score_part1 += score_values[&me][0]; // Points whether we did rock, paper or scissors
        score_part1 += score_values[&me][opponent_indexes[&opponent]]; // Points whether we won, lost or draw

        // Part 2
        let our_action: char;
        match me {
            'X' => our_action = find_pairs(opponent)[0], // Lose
            'Y' => our_action = opponent, // Draw
             _ => our_action = find_pairs(opponent)[1], // Win
        }
        score_part2 += opponent_indexes[&our_action];  // Points whether we did rock, paper or scissors
        score_part2 += action_map[&me]; // Points whether we won, lost or draw
    }
    println!("Total score using part one's strategy is: {score_part1}");
    println!("Total score using part two's strategy is: {score_part2}");
}
