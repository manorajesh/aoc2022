use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input-6.txt").unwrap();
    let letters: Vec<char> = input.chars().collect();

    let packet_length = 14;

    let mut index = 0;
    while index+packet_length-1 < letters.len() {
        let slice = &letters[index..index+packet_length];
        
        if all_unique_letters(slice) {
            println!("after {} charaters: {:?}", index+packet_length, slice);
            return
        }

        index += 1;
    }
}

fn all_unique_letters(letters: &[char]) -> bool {
    for letter in letters {
        if letters.iter().filter(|&l| *l == *letter).count() > 1 {
            return false;
        }
    }
    true
}