use std::fs::read_to_string;

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
    
    let mut head_location = [0, 0];
    let mut prev_head_location = [0, 0];
    let mut tail_location = [0, 0];
    let mut tail_past_locations: Vec<[i32; 2]> = Vec::new();
    let mut head_past_locations: Vec<[i32; 2]> = Vec::new();
    tail_past_locations.push(tail_location);
    head_past_locations.push(head_location);
    for direction in directions {
        prev_head_location = head_location;
        let dir = direction.split(" ").collect::<Vec<&str>>()[0];
        let num = direction.split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        match &dir {
            &"R" => {
                head_location[0] += num;
            }
            &"L" => {
                head_location[0] -= num;
            }
            &"U" => {
                head_location[1] += num;
            }
            &"D" => {
                head_location[1] -= num;
            }
            _ => {}
        }

        if !is_tail_touching_head(head_location, tail_location) {
            tail_location = prev_head_location;
            if !tail_past_locations.contains(&tail_location) {
                tail_past_locations.push(tail_location);
            }
        }
        head_past_locations.push(head_location);
        // print_it(&tail_past_locations);
    }
    // print_it(&tail_past_locations);
    println!("Tail locations: {}", tail_past_locations.len());
}

fn is_tail_touching_head(head_location: [i32; 2], tail_location: [i32; 2]) -> bool {
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