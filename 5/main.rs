use std::fs::read_to_string;
use std::str::Lines;

fn main() {
    let input = read_to_string("input-5.txt").unwrap();
    let lines = input.lines();

    let mut chart = get_chart(lines.clone());

    let mut directions: Vec<&str> = Vec::new();
    for line in lines {
        if line.contains("move") {
            directions.push(line);
        }
    }
    
    for direction in directions {
        let direction = direction.split(" ").collect::<Vec<&str>>();
        let from = direction[3].parse::<usize>().unwrap();
        let to = direction[5].parse::<usize>().unwrap();
        let amount = direction[1].parse::<usize>().unwrap();

        move_amount(&mut chart, from-1, to-1, amount);
    }

    let mut top_of_stacks: String = String::new();
    for line in &chart {
        top_of_stacks += &line[0].to_string();
    }
    print_chart(&chart);
    println!("{}", top_of_stacks);
}

fn print_chart(chart: &Vec<Vec<char>>) {
    for line in chart {
        println!("{:?}", line);
    }
}

fn get_chart(lines: Lines) -> Vec<Vec<char>> {
    let mut chart: Vec<Vec<char>> = Vec::new();

    // Read the chart
    for line in lines {
        if line == "" { break; }

        for (i, c) in line.chars().enumerate() {
            if chart.len() <= i {
                chart.push(Vec::new());
            }

            if c.is_alphabetic() {
                chart[i].push(c);
            }
        }
    }
    
    // Remove the empty lines
    for i in (0..chart.len()).rev() {
        if chart[i].len() == 0 {
            chart.remove(i);
        }
    }

    chart
}

fn move_amount(chart: &mut Vec<Vec<char>>, from: usize, to: usize, amount: usize) {
    for _ in 0..amount {
        let c = chart[from].remove(0);
        chart[to].insert(0, c);
    }
}