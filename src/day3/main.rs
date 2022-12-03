use std::fs;

fn find_priority(c: char) -> u8 {
    let ascii = c as u8;
    if ascii >= 97 && ascii <= 122 { // Lowercase
        ascii - 96
    }
    else if ascii >= 65 && ascii <= 90 { // Uppercase
        ascii - 38
    }
    else {
        0
    }
}

fn main() {
    let file_path: &str = "src/day3/input.txt";
    let content: String = fs::read_to_string(file_path)
        .expect("File failed to read");
    let mut priority_sum_1: usize = 0;
    let rucksacks: Vec<&str> = content.split('\n').collect();
    for ruck in rucksacks.iter() {
        let half_len: usize = ruck.len() / 2;
        if half_len < 1 {
            continue;
        }
        // Part 1
        let first: &str = &ruck[..half_len];
        let second: &str = &ruck[half_len..];
        let mut shared: Vec<char> = Vec::new();
        for item in second.chars() {
            if first.contains(item) && !shared.contains(&item) {
                shared.push(item);
                let priority: usize = find_priority(item) as usize;
                priority_sum_1 += priority;
                //println!("{item} {priority}");
            }
        }
    }

    println!("The sum of the priorities of each rucksack is: {priority_sum_1}");
}