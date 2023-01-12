use std::{
    collections::BTreeSet,
    io::{self, BufRead},
};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Block {
    x: u32,
    y: u32,
    z: u32,
}

impl Block {
    fn new(x: u32, y: u32, z: u32) -> Block {
        Block { x, y, z }
    }

    fn count_exposed(&self, blocks: &[Block]) -> u32 {
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
        (self.x == block.x
            && self.y == block.y
            && ((block.z != 0 && self.z == block.z - 1) || self.z == block.z + 1))
            || (self.x == block.x
                && self.z == block.z
                && ((block.y != 0 && self.y == block.y - 1) || self.y == block.y + 1))
            || (self.z == block.z
                && self.y == block.y
                && ((block.x != 0 && self.x == block.x - 1) || self.x == block.x + 1))
    }

    fn neighbours(&self, max_x: u32, max_y: u32, max_z: u32) -> Vec<Block> {
        let mut neighbours = vec![];
        if self.x > 0 {
            neighbours.push(Block::new(self.x - 1, self.y, self.z));
        }
        if self.x < max_x {
            neighbours.push(Block::new(self.x + 1, self.y, self.z));
        }
        if self.y > 0 {
            neighbours.push(Block::new(self.x, self.y - 1, self.z));
        }
        if self.y < max_y {
            neighbours.push(Block::new(self.x, self.y + 1, self.z));
        }
        if self.z > 0 {
            neighbours.push(Block::new(self.x, self.y, self.z - 1));
        }
        if self.z < max_z {
            neighbours.push(Block::new(self.x, self.y, self.z + 1));
        }
        neighbours
    }
}

fn line_to_block(line: &str) -> eyre::Result<Block> {
    let values: Vec<_> = line.split(',').collect();
    if values.len() != 3 {
        Err(eyre::eyre!("Invalid values split for line: {line}"))
    } else {
        Ok(Block::new(
            values[0].parse()?,
            values[1].parse()?,
            values[2].parse()?,
        ))
    }
}

fn fill_in_gaps(blocks: &[Block]) -> Vec<Block> {
    let max_x = blocks.iter().map(|b| b.x).max().unwrap() + 2;
    let max_y = blocks.iter().map(|b| b.y).max().unwrap() + 2;
    let max_z = blocks.iter().map(|b| b.z).max().unwrap() + 2;
    let mut steam = BTreeSet::<Block>::new();
    let mut new_steam = vec![Block::new(0, 0, 0)];
    while !new_steam.is_empty() {
        let next = new_steam.pop().unwrap();
        for neighbour in next.neighbours(max_x, max_y, max_z) {
            if steam.contains(&neighbour) {
                continue;
            }
            if blocks.contains(&neighbour) {
                continue;
            }
            new_steam.push(neighbour);
        }
        steam.insert(next);
    }
    let mut new_blocks = Vec::<Block>::new();
    for i in 0..max_x {
        for j in 0..max_y {
            for k in 0..max_z {
                let block = Block::new(i, j, k);
                if !steam.contains(&block) {
                    new_blocks.push(block);
                }
            }
        }
    }
    new_blocks
}

fn main() -> eyre::Result<()> {
    let mut blocks = Vec::new();
    for line in io::stdin().lock().lines() {
        let line = line?;
        blocks.push(line_to_block(&line)?);
    }
    let sum: u32 = blocks.iter().map(|b| b.count_exposed(&blocks)).sum();
    println!("Exposed Sum: {}", sum);
    let new_blocks = fill_in_gaps(&blocks);
    let sum: u32 = new_blocks
        .iter()
        .map(|b| b.count_exposed(&new_blocks))
        .sum();
    println!("Externally Exposed Sum: {}", sum);
    Ok(())
}
