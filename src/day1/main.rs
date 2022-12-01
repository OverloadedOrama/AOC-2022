use std::fs;

fn main() {
    let file_path: &str = "src/day1/input.txt";
    let content: String = fs::read_to_string(file_path)
        .expect("File failed to read");
    let elves_list: Vec<&str> = content.split("\n\n").collect();
    let mut calories_per_elf: Vec<u32> = Vec::new();

    for elf in elves_list.iter() {
        let calories_list: Vec<&str> = elf.split('\n').collect();
        let mut calories_sum: u32 = 0;
        for cal in calories_list.iter(){
            let calory = cal.parse::<u32>();
            if !calory.is_err() {
                calories_sum += calory.unwrap();
            }
        }
        calories_per_elf.push(calories_sum);
    }
    calories_per_elf.sort();
    let elf_number: usize = calories_per_elf.len();
    println!("The elf with the most calories is carrying {} calories.", calories_per_elf[elf_number - 1]);
    let top_3_calories: u32 = calories_per_elf[elf_number - 1] + calories_per_elf[elf_number - 2] + calories_per_elf[elf_number - 3];
    println!("The top three elves are carrying {top_3_calories} calories.");
}
