use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input-10.txt").unwrap();
    let lines = input.lines();
    let mut X = 1; // register
    let mut cycles = 0;
    let mut signal_strengths = Vec::new();

    let mut pause_at: usize = 20;

    for inst in lines {
        let op = inst.split(" ").nth(0).unwrap();

        match op {
            "noop" => signal_strengths.push(wait(1, &mut cycles, X, pause_at)),
            "addx" => {
                signal_strengths.push(wait(2, &mut cycles, X, pause_at)); 
                X += inst.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
            },
            _ => panic!("Unknown instruction: {}", op),
        }
        if pause_at > 220 {
            break;
        } else if cycles >= pause_at {
            pause_at += 40;
        }
    }
    println!("Signal Strengths: {:?}", signal_strengths.iter().sum::<i32>());
}

fn wait(wait_cycles: usize, current_cycle: &mut usize, reg_val: i32, pause_cycle: usize) -> i32 {
    let mut signal_strength = 0;
    for _ in 1..=wait_cycles {
        if *current_cycle == pause_cycle {
            signal_strength = pause_cycle as i32 * reg_val;
            println!("Signal Strength: {}; Cycle Count: {}; X = {}", signal_strength, &current_cycle, reg_val);
        }
        *current_cycle += 1;
    }
    if *current_cycle == pause_cycle {
        signal_strength = pause_cycle as i32 * reg_val;
        println!("Signal Strength: {}; Cycle Count: {}; X = {}", signal_strength, &current_cycle, reg_val);
    }
    signal_strength
}