use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut totals: Vec<i32> = Vec::new();
    let mut elf_total = 0;

    for line in input.lines() {
        if line == "" {
            totals.push(elf_total);
            elf_total = 0;
        } else {
            let number = line.parse::<i32>().unwrap();
            elf_total += number;
        }
    }
    totals.push(elf_total);

    let most_cals = totals.iter().max().unwrap();
    println!("Most calories: {}\n", most_cals);

    totals.sort();
    totals.reverse();
    println!("Top 3: {:?}", &totals[0..3]);
    println!("{}", &totals[0..3].iter().sum::<i32>());
}