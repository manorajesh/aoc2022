use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input-3.txt").unwrap();
    let rucksacks = input.lines();
    let mut total_priorities = 0;

    for rucksack in rucksacks {
        let compart1 = &rucksack[0..rucksack.len()/2];
        let compart2 = &rucksack[rucksack.len()/2..rucksack.len()];

        let mut shared: char = ' ';
        for char in compart1.chars() {
            if compart2.contains(char) {
                shared = char;
            }
        }

        match shared.is_lowercase() {
            true => total_priorities += (shared as u8 - 96) as i32,
            false => total_priorities += (shared as u8 - 38) as i32,
        }
    }
    println!("{}", total_priorities);
}