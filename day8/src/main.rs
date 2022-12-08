use std::io::{self, BufRead};

struct Tree {
    height: i32,
    visible: bool,
}

impl Tree {
    fn new(height: i32) -> Tree {
        Tree{
            height,
            visible: false
        }
    }
}

fn line_to_row(line: &str) -> eyre::Result<Vec<Tree>> {
    let row: Vec<_> = line.as_bytes().iter().map(|b| Tree::new(*b as i32)).collect();
    Ok(row)
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
            if tree.height> max_height {
                tree.visible = true;
                max_height = tree.height;
            }
        }
        max_height = -1;
        for mut tree in row.iter_mut().rev() {
            if tree.height> max_height {
                tree.visible = true;
                max_height = tree.height;
            }
        }
    }
    // Work out the visibility for each tree by column.
    for column in 0 .. trees[0].len() {
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

    let sum: usize = trees.iter().map(|r| r.iter().filter(|t| t.visible).count()).sum();
    println!("Sum: {}", sum);
    Ok(())
}
