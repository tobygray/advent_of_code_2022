use std::{io::{self, BufRead}, collections::{VecDeque, BTreeSet}};

fn find_unique_offset(bytes: &[u8]) -> eyre::Result<usize> {
    let mut last = VecDeque::new();
    for (idx, byte) in bytes.iter().enumerate() {
        if last.len() < 3 {
            last.push_back(byte);
            continue;
        }
        last.push_back(byte);
        if last.len() > 4 {
            last.pop_front();
        }
        assert!(last.len() == 4);
        let byte_set: BTreeSet<_> = last.iter().collect();
        if byte_set.len() == 4 {
            return Ok(idx + 1)
        }
    }
    Err(eyre::eyre!("Failed to find start"))
}

fn main() -> eyre::Result<()> {
    for line in io::stdin().lock().lines() {
        let line = line?;
        let unique_offset = find_unique_offset(line.as_bytes())?;
        println!("Offset: {}", unique_offset);
    }
    Ok(())
}
