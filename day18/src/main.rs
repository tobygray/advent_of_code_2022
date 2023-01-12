use std::io::{self, BufRead};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Block {
    x: i32,
    y: i32,
    z: i32,
}

impl Block {
    fn new(x: i32, y: i32, z: i32) -> Block {
        Block{x, y, z}
    }

    fn count_exposed(&self, blocks: &[Block]) -> i32 {
        let mut sides_exposed = 6;
        for block in blocks {
            if block == self {
                continue;
            }
            if self.next_to(block) {
                sides_exposed -= 1;
            }
        }
        sides_exposed
    }

    fn next_to(&self, block: &Block) -> bool {
        (self.x == block.x && self.y == block.y && (self.z == block.z - 1 || self.z == block.z + 1)) ||
            (self.x == block.x && self.z == block.z && (self.y == block.y - 1 || self.y == block.y + 1)) ||
            (self.z == block.z && self.y == block.y && (self.x == block.x - 1 || self.x == block.x + 1))
    }
}

fn line_to_block(line: &str) -> eyre::Result<Block> {
    let values: Vec<_> = line.split(',').collect();
    if values.len() != 3 {
        Err(eyre::eyre!("Invalid values split for line: {line}"))
    } else {
        Ok(Block::new(values[0].parse()?, values[1].parse()?, values[2].parse()?))
    }
}

fn main() -> eyre::Result<()> {
    let mut blocks = Vec::new();
    for line in io::stdin().lock().lines() {
        let line = line?;
        blocks.push(line_to_block(&line)?);
    }
    let sum: i32 = blocks.iter().map(|b| b.count_exposed(&blocks)).sum();
    println!("Exposed Sum: {}", sum);
    Ok(())
}
