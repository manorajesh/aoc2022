use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input-4.txt").unwrap();
    let lines = input.lines();
    let mut overlaps = 0;

    for line in lines {
        let ranges = line.replace(",", "-");
        let ranges: Vec<&str> = ranges.split("-").collect();

        if is_overlap(ranges[0].parse().unwrap(), ranges[1].parse().unwrap(), ranges[2].parse().unwrap(), ranges[3].parse().unwrap()) {
            overlaps += 1;
        }
    }
    println!("{}", overlaps);
}

fn is_in_range(start1: i32, end1: i32, start2: i32, end2: i32) -> bool {
    start1 <= start2 && end1 >= end2
}

fn is_overlap(start1: i32, end1: i32, start2: i32, end2: i32) -> bool {
    end1 >= start2 && end2 >= start1
}

// .234567..  2-7
// .....678.  6-8

// 6-8,2-7
// 6 >= 2
// 7 <= 6
