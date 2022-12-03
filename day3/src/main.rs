use std::{
    collections::BTreeSet,
    io::{self, BufRead},
};

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
    let mut current_group = Vec::new();
    for line in io::stdin().lock().lines() {
        let line = line?;
        let items = to_item_priority_set(&line)?;
        current_group.push(items);
        if current_group.len() == 3 {
            let intersection = current_group
                .iter()
                .skip(1)
                .fold(current_group[0].clone(), |acc, hs| {
                    acc.intersection(hs).cloned().collect()
                });
            if intersection.len() != 1 {
                return Err(eyre::eyre!("Unexpected intersection {:?}", intersection));
            }
            priority_sum += intersection.iter().next().unwrap();
            current_group.clear();
        }
    }
    println!("Priority sum: {}", priority_sum);
    Ok(())
}
