use std::io::{self, BufRead};

struct Tree {
    height: i32,
    visible: bool,
}

impl Tree {
    fn new(height: i32) -> Tree {
        Tree {
            height,
            visible: false,
        }
    }
}

fn line_to_row(line: &str) -> eyre::Result<Vec<Tree>> {
    let row: Vec<_> = line
        .as_bytes()
        .iter()
        .map(|b| Tree::new(*b as i32))
        .collect();
    Ok(row)
}

fn score_row(trees: &[Vec<Tree>], row: usize, column: usize, dir: i32) -> i32 {
    let tree_height = trees[row][column].height;
    let mut current_idx = column as i32;
    while current_idx != 0 && current_idx as usize != trees[row].len() - 1 {
        if trees[row][(current_idx + dir) as usize].height >= tree_height {
            current_idx += dir;

            break;
        }
        current_idx += dir;
    }
    column.abs_diff(current_idx as usize) as i32
}

fn score_column(trees: &[Vec<Tree>], row: usize, column: usize, dir: i32) -> i32 {
    let tree_height = trees[row][column].height;
    let mut current_idx = row as i32;
    while current_idx != 0 && current_idx as usize != trees.len() - 1 {
        if trees[(current_idx + dir) as usize][column].height >= tree_height {
            current_idx += dir;

            break;
        }
        current_idx += dir;
    }
    row.abs_diff(current_idx as usize) as i32
}

fn main() -> eyre::Result<()> {
    let mut trees = Vec::<Vec<Tree>>::new();
    for line in io::stdin().lock().lines() {
        let line = line?;
        let row = line_to_row(&line)?;
        trees.push(row);
    }
    // Work out the visibility for each tree by row.
    for row in trees.iter_mut() {
        let mut max_height = -1;
        for mut tree in row.iter_mut() {
            if tree.height > max_height {
                tree.visible = true;
                max_height = tree.height;
            }
        }
        max_height = -1;
        for mut tree in row.iter_mut().rev() {
            if tree.height > max_height {
                tree.visible = true;
                max_height = tree.height;
            }
        }
    }
    // Work out the visibility for each tree by column.
    for column in 0..trees[0].len() {
        let mut max_height = -1;
        for row in trees.iter_mut() {
            let tree = &mut row[column];
            if tree.height > max_height {
                tree.visible = true;
                max_height = tree.height;
            }
        }
        max_height = -1;
        for row in trees.iter_mut().rev() {
            let tree = &mut row[column];
            if tree.height > max_height {
                tree.visible = true;
                max_height = tree.height;
            }
        }
    }

    let sum: usize = trees
        .iter()
        .map(|r| r.iter().filter(|t| t.visible).count())
        .sum();
    println!("Sum of trees: {}", sum);

    let mut max_scenic_score = -1;
    for row in 0..trees.len() {
        for column in 0..trees[row].len() {
            let score_left = score_row(&trees, row, column, -1);
            let score_right = score_row(&trees, row, column, 1);
            let score_up = score_column(&trees, row, column, -1);
            let score_down = score_column(&trees, row, column, 1);
            let scenic_score = score_left * score_right * score_up * score_down;
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }
    println!("Max scenic score: {}", max_scenic_score);
    Ok(())
}
