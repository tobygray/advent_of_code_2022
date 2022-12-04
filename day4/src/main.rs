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

fn contains(range1: (i32, i32), range2: (i32, i32)) -> bool {
    range1.0 >= range2.0 && range1.1 <= range2.1
}

fn overlaps(range1: (i32, i32), range2: (i32, i32)) -> bool {
    (range1.0 >= range2.0 && range1.0 <= range2.1) || (range1.1 >= range2.0 && range1.1 <= range2.1)
}

fn main() -> eyre::Result<()> {
    let mut sum = 0;
    for line in io::stdin().lock().lines() {
        let line = line?;
        let (elf1, elf2) = line_to_ranges(&line)?;
        if contains(elf1, elf2)
            || contains(elf2, elf1)
            || overlaps(elf1, elf2)
            || overlaps(elf2, elf1)
        {
            sum += 1;
        }
    }
    println!("Sum: {}", sum);
    Ok(())
}
