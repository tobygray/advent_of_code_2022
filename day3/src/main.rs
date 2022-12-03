use std::io::{self, BufRead};

fn main() -> eyre::Result<()> {
    let priority_sum = 0;
    for _line in io::stdin().lock().lines() {
    }
    println!("Priority sum: {}", priority_sum);
    Ok(())
}
