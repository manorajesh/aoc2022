use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input-10.txt").unwrap();
    let lines = input.lines();
    let mut X = 1; // register
    let mut cycles = 0;
    let mut crt_frame_buffer = Vec::with_capacity(240);

    let mut pause_at: usize = 0;

    for inst in lines {
        let op = inst.split(" ").nth(0).unwrap();

        match op {
            "noop" => draw(1, &mut cycles, X, &mut crt_frame_buffer),
            "addx" => {
                draw(2, &mut cycles, X, &mut crt_frame_buffer); 
                X += inst.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
            },
            _ => panic!("Unknown instruction: {}", op),
        }
        if pause_at > 240 {
            break;
        } else if cycles >= pause_at {
            pause_at += 40;
        }
        // clear_screen();
        // print_crt(&crt_frame_buffer);
    }
    print_crt(&crt_frame_buffer);
}

fn draw(wait_cycles: usize, current_cycle: &mut usize, reg_val: i32, crt_frame_buffer: &mut Vec<char>) {
    for _ in 1..=wait_cycles {
        *current_cycle += 1;
        let crt_length = crt_frame_buffer.len() as i32 % 40;
        if crt_length >= reg_val-1 && crt_length <= reg_val+1 {
            crt_frame_buffer.push('#');
        } else {
            crt_frame_buffer.push('.');
        }
        // println!("CRT: {:?}\nSprite: ", crt_frame_buffer);
        for i in 0..40 {
            if i >= reg_val -1 && i <= reg_val+1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        // println!("\n");
    }
}

fn print_crt(frame_buffer: &Vec<char>) {
    for (i, letter) in frame_buffer.iter().enumerate() {
        if i % 40 == 0 {
            println!();
        } 
        print!("{letter}");
    }
    println!();
}

fn clear_screen() {
    print!("\x1b2J");
}