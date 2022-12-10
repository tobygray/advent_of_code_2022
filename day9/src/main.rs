use std::{
    collections::BTreeSet,
    io::{self, BufRead},
};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn line_to_move(line: &str) -> eyre::Result<(Direction, i32)> {
    let (direction, count) = line
        .split_once(' ')
        .ok_or_else(|| eyre::eyre!("Unable to split line {}", line))?;
    let direction = match direction {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => return Err(eyre::eyre!("Invalid direction: {}", direction)),
    };
    let count = count.parse()?;
    Ok((direction, count))
}

fn main() -> eyre::Result<()> {
    let mut tail_positions = BTreeSet::<(i32, i32)>::new();
    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);
    tail_positions.insert(tail_pos);
    for line in io::stdin().lock().lines() {
        let line = line?;
        let (direction, count) = line_to_move(&line)?;
        let delta = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        for _ in 0..count {
            head_pos = (head_pos.0 + delta.0, head_pos.1 + delta.1);
            if tail_pos.0 < head_pos.0 - 1
                || tail_pos.0 > head_pos.0 + 1
                || tail_pos.1 < head_pos.1 - 1
                || tail_pos.1 > head_pos.1 + 1
            {
                // Tail needs to move closer to the head.
                if tail_pos.0 > head_pos.0 {
                    tail_pos.0 -= 1;
                }
                if tail_pos.0 < head_pos.0 {
                    tail_pos.0 += 1;
                }
                if tail_pos.1 > head_pos.1 {
                    tail_pos.1 -= 1;
                }
                if tail_pos.1 < head_pos.1 {
                    tail_pos.1 += 1;
                }
            }
            tail_positions.insert(tail_pos);
        }
    }
    println!("Total unique tail positions: {}", tail_positions.len());
    Ok(())
}
