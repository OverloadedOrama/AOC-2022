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
    let mut priority_sum_2: usize = 0;
    let rucksacks: Vec<&str> = content.split('\n').collect();
    // Part 1
    for ruck in rucksacks.iter() {
        let half_len: usize = ruck.len() / 2;
        if half_len < 1 {
            continue;
        }     
        let first: &str = &ruck[..half_len];
        let second: &str = &ruck[half_len..];
        let mut shared: Vec<char> = Vec::new();
        for item in second.chars() {
            if first.contains(item) && !shared.contains(&item) {
                shared.push(item);
                let priority: usize = find_priority(item) as usize;
                priority_sum_1 += priority;
            }
        }
    }
    // Part 2
    let step: usize = 3;
    for i in ((step - 1)..rucksacks.len()).step_by(step) { // Iterate through the vector with a step of 3, starting at index 2
        let first: &str = rucksacks[i - 2];
        let second: &str = rucksacks[i - 1];
        let third: &str = rucksacks[i];
        if third.len() < 1 {
            continue
        }
        let mut shared: char = '0'; // Temporary placeholder char
        for item in third.chars() {
            if first.contains(item) && second.contains(item) && shared != item {
                shared = item;
            }
        }
        let priority: usize = find_priority(shared) as usize;
        priority_sum_2 += priority;
    }
    println!("The sum of the priorities of each rucksack is: {priority_sum_1}");
    println!("The sum of the priorities of each group is: {priority_sum_2}");
}