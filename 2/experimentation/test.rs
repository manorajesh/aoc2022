use std::collections::HashMap;

fn main() {
    let win_table = HashMap::from([
        ('A', 'C'),
        ('B', 'A'),
        ('C', 'B'),
    ]);
    let home = win_table.iter().find(|(_, &v)| v == 'C').unwrap().0;
    println!("{}", home);

}