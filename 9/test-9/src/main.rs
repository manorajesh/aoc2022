use std::fs::read_to_string;

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

            if !is_knot_touching_other_knot([prev_knot.x, prev_knot.y], [knot.x, knot.y]) {
                knot.prev_x = knot.x;
                knot.prev_y = knot.y;
                knot.x = prev_knot.prev_y;
                knot.y = prev_knot.prev_y;

                if distance(*knot, prev_knot) > 1 {
                    let x_diff = (knot.x - prev_knot.x).abs();
                    let y_diff = (knot.y - prev_knot.y).abs();
                    if x_diff > y_diff {
                        if knot.x > prev_knot.x {
                            knot.x = prev_knot.x + 1;
                        } else {
                            knot.x = prev_knot.x - 1;
                        }
                    } else {
                        if knot.y > prev_knot.y {
                            knot.y = prev_knot.y + 1;
                        } else {
                            knot.y = prev_knot.y - 1;
                        }
                    }
                }

                if i == knots_len-1 && !tail_past_locations.contains(&[knot.x, knot.y]){
                    tail_past_locations.push([knot.x, knot.y]);
                }
            }
        }
        print_it(&tail_past_locations);
    }
    print_it(&tail_past_locations);
    println!("Tail locations: {}", tail_past_locations.len());
}

fn distance(loc1: Knot, loc2: Knot) -> i32 {
    ((loc1.x - loc2.y).abs() + (loc1.x - loc2.y).abs())
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