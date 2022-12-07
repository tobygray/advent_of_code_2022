use std::{io::{self, BufRead}, collections::{VecDeque, BTreeSet}};

const HEADER_LENGTH: usize = 14;

fn find_unique_offset(bytes: &[u8]) -> eyre::Result<usize> {
    let mut last = VecDeque::new();
    for (idx, byte) in bytes.iter().enumerate() {
        if last.len() < HEADER_LENGTH - 1 {
            last.push_back(byte);
            continue;
        }
        last.push_back(byte);
        if last.len() > HEADER_LENGTH {
            last.pop_front();
        }
        assert!(last.len() == HEADER_LENGTH);
        let byte_set: BTreeSet<_> = last.iter().collect();
        if byte_set.len() == HEADER_LENGTH {
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
