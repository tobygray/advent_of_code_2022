use std::io::{self, BufRead};

fn range_to_tuple(range: &str) -> eyre::Result<(i32, i32)> {
    let (lower, upper) = range
        .split_once('-')
        .ok_or_else(|| eyre::eyre!("Failed to split on -: {}", range))?;
    Ok((lower.parse::<i32>()?, upper.parse::<i32>()?))
}

fn line_to_ranges(line: &str) -> eyre::Result<((i32, i32), (i32, i32))> {
    let (elf1_str, elf2_str) = line
        .split_once(',')
        .ok_or_else(|| eyre::eyre!("Failed to split on ,: {}", line))?;
    let elf1 = range_to_tuple(elf1_str)?;
    let elf2 = range_to_tuple(elf2_str)?;
    Ok((elf1, elf2))
}

fn main() -> eyre::Result<()> {
    let mut sum = 0;
    for line in io::stdin().lock().lines() {
        let line = line?;
        let (elf1, elf2) = line_to_ranges(&line)?;
        if (elf1.0 >= elf2.0 && elf1.1 <= elf2.1) || (elf2.0 >= elf1.0 && elf2.1 <= elf1.1) {
            sum += 1;
        }
    }
    println!("Sum: {}", sum);
    Ok(())
}
