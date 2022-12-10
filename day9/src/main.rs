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
    let mut knots: Vec<(i32, i32)> = (0..10).map(|_| (0, 0)).collect();
    tail_positions.insert(knots[knots.len() - 1]);
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
            knots[0] = (knots[0].0 + delta.0, knots[0].1 + delta.1);
            for i in 1..knots.len() {
                if knots[i].0 < knots[i - 1].0 - 1
                    || knots[i].0 > knots[i - 1].0 + 1
                    || knots[i].1 < knots[i - 1].1 - 1
                    || knots[i].1 > knots[i - 1].1 + 1
                {
                    // Tail needs to move closer to the head.
                    if knots[i].0 > knots[i - 1].0 {
                        knots[i].0 -= 1;
                    }
                    if knots[i].0 < knots[i - 1].0 {
                        knots[i].0 += 1;
                    }
                    if knots[i].1 > knots[i - 1].1 {
                        knots[i].1 -= 1;
                    }
                    if knots[i].1 < knots[i - 1].1 {
                        knots[i].1 += 1;
                    }
                }
            }
            tail_positions.insert(knots[knots.len() - 1]);
        }
    }
    println!("Total unique tail positions: {}", tail_positions.len());
    Ok(())
}
