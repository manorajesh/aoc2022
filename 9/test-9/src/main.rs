use std::fs::read_to_string;
use std::ops::Sub;

#[derive(Debug, Clone, Copy)]
struct Knot {
    x: i32,
    y: i32,
    prev_x: i32,
    prev_y: i32,
}

impl Knot {
    fn new() -> Knot {
        Knot {
            x: 0,
            y: 0,
            prev_x: 0,
            prev_y: 0,
        }
    }
}

impl Sub for Knot {
    type Output = Self;

    fn sub(self, other: Knot) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            prev_x: self.prev_x,
            prev_y: self.prev_y,
        }
    }
}

fn main() {
    let input = read_to_string("input-9.txt").unwrap();
    let lines = input.lines();
    let mut directions: Vec<String> = Vec::new();

    for line in lines {
        let num = line.split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        if num > 1 {
            for _ in 0..num {
                directions.push(line.replace(&num.to_string(), "1").to_string());
            }
        } else {
            directions.push(line.to_string());
        }
    }
    
    let mut tail_past_locations: Vec<[i32; 2]> = Vec::new();
    let mut knots: Vec<Knot> = Vec::new();
    for _ in 0..9 {
        knots.push(Knot::new());
    }
    let knots_len = knots.len();

    tail_past_locations.push([0, 0]);
    for direction in directions {
        knots[0].prev_x = knots[0].x;
        knots[0].prev_y = knots[0].y;
        let dir = direction.split(" ").collect::<Vec<&str>>()[0];
        let num = direction.split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        match &dir {
            &"R" => {
                knots[0].x += num;
            }
            &"L" => {
                knots[0].x -= num;
            }
            &"U" => {
                knots[0].y += num;
            }
            &"D" => {
                knots[0].y -= num;
            }
            _ => {}
        }

        for i in 1..knots_len {
            let prev_knot = knots[i-1].clone();
            let mut knot = &mut knots[i];

            let diff = prev_knot - *knot;
            let (dx, dy) = match (diff.x, diff.y) {
                // overlapping
                (0, 0) => (0, 0),
                // touching up/left/down/right
                (0, 1) | (1, 0) | (0, -1) | (-1, 0) => (0, 0),
                // touching diagonally
                (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => (0, 0),
                // need to move up/left/down/right
                (0, 2) => (0, 1),
                (0, -2) => (0, -1),
                (2, 0) => (1, 0),
                (-2, 0) => (-1, 0),
                // need to move to the right diagonally
                (2, 1) => (1, 1),
                (2, -1) => (1, -1),
                // need to move to the left diagonally
                (-2, 1) => (-1, 1),
                (-2, -1) => (-1, -1),
                // need to move up/down diagonally
                (1, 2) => (1, 1),
                (-1, 2) => (-1, 1),
                (1, -2) => (1, -1),
                (-1, -2) => (-1, -1),
                // 🆕 need to move diagonally
                (-2, -2) => (-1, -1),
                (-2, 2) => (-1, 1),
                (2, -2) => (1, -1),
                (2, 2) => (1, 1),
                _ => panic!("unhandled case: tail - head = {diff:?}"),
            };
            knot.x += dx;
            knot.y += dy;

            if i == knots_len - 1 && !tail_past_locations.contains(&[knot.x, knot.y]) {
                tail_past_locations.push([knot.x, knot.y]);
            }
        }

        // print_it(&tail_past_locations);
    }
    // print_it(&tail_past_locations);
    println!("Tail locations: {}", tail_past_locations.len());
}

fn is_knot_touching_other_knot(head_location: [i32; 2], tail_location: [i32; 2]) -> bool {
    let x_diff = (head_location[0] - tail_location[0]).abs();
    let y_diff = (head_location[1] - tail_location[1]).abs();

    if x_diff < 2 && y_diff < 2 {
        true
    } else {
        false
    }
}

fn print_it(tail_past_locations: &Vec<[i32; 2]>) {
    let min_x = tail_past_locations.iter().map(|x| x[0]).min().unwrap();
    let max_x = tail_past_locations.iter().map(|x| x[0]).max().unwrap();
    let min_y = tail_past_locations.iter().map(|x| x[1]).min().unwrap();
    let max_y = tail_past_locations.iter().map(|x| x[1]).max().unwrap();

    clear_screen();

    for y in ((min_y-1)..(max_y+2)).rev() {
        for x in (min_x-1)..(max_x+2) {
            if tail_past_locations.contains(&[x, y]) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn clear_screen() {
    print!("\x1b[2J");
}