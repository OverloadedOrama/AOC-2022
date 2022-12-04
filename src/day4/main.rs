use std::fs;


fn main() {
    let file_path: &str = "src/day4/input.txt";
    let content: String = fs::read_to_string(file_path)
        .expect("File failed to read");
    let mut fully_contained_pairs: usize = 0;
    let mut number_of_overlapping: usize = 0;
    let pairs: Vec<&str> = content.split('\n').collect();
    for pair in pairs.iter() {
        if pair.len() < 1 {
            continue;
        }
        let elves: Vec<&str> = pair.split(',').collect();
        let first_elf: Vec<&str> = elves[0].split('-').collect();
        let second_elf: Vec<&str> = elves[1].split('-').collect();

        let first_elf_lower: usize = first_elf[0].parse::<usize>().unwrap();
        let first_elf_upper: usize = first_elf[1].parse::<usize>().unwrap();
        let second_elf_lower: usize = second_elf[0].parse::<usize>().unwrap();
        let second_elf_upper: usize = second_elf[1].parse::<usize>().unwrap();

        if first_elf_lower <= second_elf_lower && first_elf_upper >= second_elf_upper { // If the first elf fully contains the second
            fully_contained_pairs += 1;
        }
        else if second_elf_lower <= first_elf_lower && second_elf_upper >= first_elf_upper { // If the second elf fully contains the first
            fully_contained_pairs += 1;
        }
    }
    println!("There are {fully_contained_pairs} pairs where one range fully contains the other");
}
