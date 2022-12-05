use std::io::{self, BufRead};

fn line_to_moves(line: &str) -> eyre::Result<(usize, usize, usize)> {
    let words: Vec<&str> = line.split(' ').collect();
    Ok((words[1].parse()?, words[3].parse()?, words[5].parse()?))
}

fn main() -> eyre::Result<()> {
    let mut reading_initial_state = true;
    // Cheat and assume number of stacks in known input.
    let mut crate_stacks: Vec<Vec<char>> = (0..9).map(|_| Vec::new()).collect();
    for line in io::stdin().lock().lines() {
        let line = line?;
        if reading_initial_state {
            let chars: Vec<_> = line.chars().collect();
            if !chars.is_empty() && chars[0] == '[' {
                for (idx, stack) in crate_stacks.iter_mut().enumerate() {
                    let char = chars[1 + 4 * idx];
                    if char != ' ' {
                        stack.insert(0, char);
                    }
                }
            } else if line.is_empty() {
                reading_initial_state = false;
            }
            continue;
        }
        // Processing sequence of moves.
        let (count, from, to) = line_to_moves(&line)?;
        let from = &mut crate_stacks[from - 1];
        let mut crates = Vec::new();
        for crate_to_move in from.drain(from.len() - count..from.len()) {
            crates.push(crate_to_move);
        }
        let to = &mut crate_stacks[to - 1];
        to.append(&mut crates)
    }
    print!("Stack tops: ");
    for stack in crate_stacks.iter() {
        print!("{}", stack[stack.len() - 1]);
    }
    println!();
    Ok(())
}
