use std::io::{self, BufRead};

enum Instruction {
    Noop,
    AddX(i32),
}

fn line_to_instruction(line: &str) -> eyre::Result<Instruction> {
    if line == "noop" {
        Ok(Instruction::Noop)
    } else if line.starts_with("addx ") {
        let (_, immediate) = line
            .split_once(' ')
            .ok_or_else(|| eyre::eyre!("Unexpected addx format: {}", line))?;
        Ok(Instruction::AddX(immediate.parse()?))
    } else {
        Err(eyre::eyre!("Unexpected instruction: {}", line))
    }
}

fn check_signal(cycle: i32, x: i32) -> i32 {
    let offset = (cycle - 1) % 40;
    if x - 1 <= offset && x + 1 >= offset {
        print!("#");
    } else {
        print!(".");
    }
    if offset == 39 {
        println!();
    }
    if [20, 60, 100, 140, 180, 220].contains(&cycle) {
        cycle * x
    } else {
        0
    }
}

fn main() -> eyre::Result<()> {
    let mut sum = 0;
    let mut cycle = 1;
    let mut x = 1;
    check_signal(cycle, x);
    for line in io::stdin().lock().lines() {
        let line = line?;
        match line_to_instruction(&line)? {
            Instruction::Noop => {
                cycle += 1;
                sum += check_signal(cycle, x);
            }
            Instruction::AddX(count) => {
                cycle += 1;
                sum += check_signal(cycle, x);
                cycle += 1;
                x += count;
                sum += check_signal(cycle, x);
            }
        }
    }
    println!("Sum: {}", sum);
    Ok(())
}
