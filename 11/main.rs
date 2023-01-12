use std::fs::read_to_string;

struct Monkey {
    items: Vec<i32>,
    operation: &str,
    test_condition: i32, // divisible by
    if_true: usize, // which monkey to throw
    if_false: usize, // which monkey to throw
    inspected_times: usize,
}

impl Monkey {
    fn new(items: Vec<i32>, operation: &str, test_condition: i32, if_true: usize, if_false: usize) -> Monkey {
        Monkey {
            items,
            operation,
            test_condition,
            if_true,
            if_false,
            inspected_times: 0,
        }
    }
    
    fn operation(&self, item: &i32) -> usize { // return idx of monkey to throw to
        self.inspected_times += 1;
        // parse operation
        if self.test(item) {
            return self.if_true;
        } else {
            return self.if_false;
        }
    }

    fn test(&self, item: &i32) -> bool {
        if item % self.test_condition == 0 {
            return true;
        } else {
            return false;
        }
    }
}

fn main() {
    let input = read_to_string("input-11.txt").unwrap();
    let lines = input.lines();
}