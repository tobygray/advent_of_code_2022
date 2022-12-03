use std::{io::{self, BufRead}, collections::BTreeSet};

fn byte_to_priority(item: &u8) -> eyre::Result<i32> {
    let item_as_char = char::from(*item);
    if ('a'..='z').contains(&item_as_char) {
        Ok(*item as i32 - 'a' as i32 + 1)
    } else if ('A'..='Z').contains(&item_as_char) {
        Ok(*item as i32 - 'A' as i32 + 27)
    } else {
        Err(eyre::eyre!("Unexpected byte: {}", item))
    }
}

fn to_item_priority_set(item: &str) -> eyre::Result<BTreeSet<i32>> {
    item
    // Stream as bytes.
    .as_bytes()
    .iter()
    // Convert byte value to priority
    .map(byte_to_priority)
    .collect()
}


fn main() -> eyre::Result<()> {
    let mut priority_sum = 0;
    for line in io::stdin().lock().lines() {
        let line = line?;
        if line.len() % 2 != 0 {
            return Err(eyre::eyre!("Line isn't an even length: {}", line))
        }
        let large_compartment = to_item_priority_set(&line[0 .. line.len() / 2])?;
        let small_compartment = to_item_priority_set(&line[line.len() / 2 .. line.len()])?;
        let difference: Vec<_> = large_compartment.intersection(&small_compartment).collect();
        if difference.len() != 1 {
            return Err(eyre::eyre!("Unexpected difference length {:?} of '{:?}' vs '{:?}'", difference, large_compartment, small_compartment))
        }
        priority_sum += difference[0];
    }
    println!("Priority sum: {}", priority_sum);
    Ok(())
}
