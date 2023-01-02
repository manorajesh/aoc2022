// A defeats C  1 defeats 3
// B defeats A  2 defeats 1
// C defeats B  3 defeats 2

use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input-2.txt").unwrap();
    let games = input.lines();
    let win_table = HashMap::from([
            ('A', 'C'),
            ('B', 'A'),
            ('C', 'B'),
        ]);
    let mut total_score = 0;

    for game in games {
        let away_move = game.chars().nth(0).unwrap();
        let round_ending = game.chars().nth(2).unwrap();
        let mut home_move: char;

        if round_ending == 'X' {
            // lose by getting value from key
            home_move = *win_table.get(&away_move).unwrap();

        } else if round_ending == 'Y' {
            // draw
            home_move = away_move;
            total_score += 3;

        } else {
            // win by getting key from value
            home_move = *win_table.iter().find(|(_, &v)| v == away_move).unwrap().0;
            total_score += 6;

        }

        match home_move {
            'A' => total_score += 1,
            'B' => total_score += 2,
            'C' => total_score += 3,
            _ => panic!(),
        }       
    }

    println!("{}", total_score);
}