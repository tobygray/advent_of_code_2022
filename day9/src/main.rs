use std::io::{self, BufRead};

fn main() -> eyre::Result<()> {
    let sum = 0;
    for line in io::stdin().lock().lines() {
        let line = line?;
        print!("{}", line);
    }
    println!("Sum: {}", sum);
    Ok(())
}
