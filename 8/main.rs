use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input-8.txt").unwrap();
    let lines = input.lines();
    let mut tree_grid: Vec<Vec<u32>> = Vec::new();

    for line in lines {
        let row = line.chars().flat_map(|x| x.to_digit(10)).collect();
        tree_grid.push(row);
    }

    let mut visible_trees = 0;
    for (i, row) in tree_grid.iter().enumerate() {
        for (j, tree) in row.iter().enumerate() {
            if is_visible(&tree_grid, (i, j)) {
                visible_trees += 1;
                print!("\x1b[32m{}\x1b[0m", tree);
            } else {
                print!("{}", tree);
            }
        }
        println!();
    }
    println!("{visible_trees} visible trees");

    let highest_score = get_highest_scenic_score(&tree_grid);
    println!("Highest score: {}", highest_score);
}

fn is_visible(trees: &Vec<Vec<u32>>, tree_idx: (usize, usize)) -> bool {
    if tree_idx.0 == 0 || tree_idx.1 == 0 || tree_idx.0 == trees.len() - 1 || tree_idx.1 == trees[0].len() - 1 {
        return true;
    }

    // check all trees in the same row and column if they are visible
    let (i, j) = tree_idx;
    let row = &trees[i];
    let column = trees.iter().map(|x| x[j]).collect::<Vec<u32>>();

    let current_tree = row[j];
    let row_visible_left = row[..j].iter().all(|x| x < &current_tree);
    let column_visible_top = column[..i].iter().all(|x| x < &current_tree);
    let row_visible = row_visible_left || row[j + 1..].iter().all(|x| x < &current_tree);
    let column_visible = column_visible_top || column[i + 1..].iter().all(|x| x < &current_tree);

    if row_visible || column_visible {
        return true;
    }
    false
}

fn get_highest_scenic_score(trees: &Vec<Vec<u32>>) -> u32 {
    let mut highest_score = 0;
    for (i, row) in trees.iter().enumerate() {
        for (j, tree) in row.iter().enumerate() {
            let score = scenic_score(&trees, (i, j));
            if score > highest_score {
                highest_score = score;
                // print!("\x1b[32m{}\x1b[0m", score);
            } else {
                // print!("{}", score);
            }
        }
        // println!();
    }
    highest_score
}

fn scenic_score(trees: &Vec<Vec<u32>>, tree_idx: (usize, usize)) -> u32 {
    if tree_idx.0 == 0 || tree_idx.1 == 0 || tree_idx.0 == trees.len() - 1 || tree_idx.1 == trees[0].len() - 1 {
        return 0;
    }

    let (i, j) = tree_idx;
    let row = &trees[i];
    let column = trees.iter().map(|x| x[j]).collect::<Vec<u32>>();

    // start from current_tree and step outward until we find a tree that is higher than the current tree or the edge of the grid
    let current_tree = row[j];
    let mut row_visible_left = 0;
    let mut row_visiable_right = 0;
    let mut column_visible_top = 0;
    let mut column_visible_bottom = 0;

    for k in (0..j).rev() {
        row_visible_left += 1;
        if row[k] >= current_tree {
            break;
        }
    }

    for k in j + 1..row.len() {
        row_visiable_right += 1;
        if row[k] >= current_tree {
            break;
        }
    }

    for k in (0..i).rev() {
        column_visible_top += 1;
        if column[k] >= current_tree {
            break;
        }
    }

    for k in i + 1..column.len() {
        column_visible_bottom += 1;
        if column[k] >= current_tree {
            break;
        }
    }

    (row_visible_left * column_visible_top * row_visiable_right * column_visible_bottom) as u32

}