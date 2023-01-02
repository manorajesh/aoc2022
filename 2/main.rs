// A defeats C  1 defeats 3
// B defeats A  2 defeats 1
// C defeats B  3 defeats 2

fn main() {
    let input = std::fs::read_to_string("input-2.txt").unwrap()
        .replace("X", "A")
        .replace("Y", "B")
        .replace("Z", "C");
    let games = input.lines();
    let mut total_score = 0;

    for game in games {
        let away_move = game.chars().nth(0).unwrap();
        let home_move = game.chars().nth(2).unwrap();

        if home_move != away_move {
            if iswin(home_move, away_move) {
                total_score += 6;
            }
        } else {
            total_score += 3;
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

fn iswin(a: char, b: char) -> bool {
    match (a, b) {
        ('A', 'C') => true,
        ('B', 'A') => true,
        ('C', 'B') => true,
        _ => false,
    }
}