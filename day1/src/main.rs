use std::io::{self, BufRead};
use std::vec::Vec;

fn main() -> eyre::Result<()> {
    let mut elves = Vec::new();
    let stdin = io::stdin();
    let mut current_elf = 0;
    for line in stdin.lock().lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            elves.push(current_elf);
            current_elf = 0;
        } else {
            current_elf += line.parse::<i32>()?;
        }
    }
    elves.push(current_elf);
    elves.sort();
    println!("Max: {0}", elves.iter().max().unwrap());
    println!("Top 3: {0}", elves[elves.len() - 3 .. elves.len()].iter().sum::<i32>());
    Ok(())
}
