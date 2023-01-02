use std::fs::read_to_string;
use std::str::Lines;

fn main() {
    let input = read_to_string("input-3.txt").unwrap();
    let rucksacks = input.lines();
    let mut total_priorities: usize = 0;

    let groups_of_3 = groups_of(rucksacks, 3);
    for group in groups_of_3 {
        let shared = find_shared_char(group[0], group[1], group[2]);

        match shared.is_lowercase() {
            true => total_priorities += (shared as u8 - 96) as usize,
            false => total_priorities += (shared as u8 - 38) as usize,
        }
    }
    println!("{}", total_priorities);
}

fn groups_of(rucksacks: Lines, group_size: usize) -> Vec<Vec<&str>> {
    let mut groups = Vec::new();
    let mut index = 1;
    let mut group = Vec::new();
    for rucksack in rucksacks {
        group.push(rucksack);
        if index > group_size-1 {
            groups.push(group);
            group = Vec::new();
            index = 0;
        }
        index += 1;
    }
    groups
}

fn find_shared_char(string1: &str, string2: &str, string3: &str) -> char {
    let mut shared = ' ';
    for char in string1.chars() {
        if string2.contains(char) && string3.contains(char) {
            shared = char;
            break;
        }
    }
    shared
}