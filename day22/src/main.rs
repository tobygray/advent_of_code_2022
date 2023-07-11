use regex::Regex;
use std::io::{self, BufRead};

enum Square {
    Empty,
    Open,
    Wall,
}

enum Instructions {
    Forward(count: i32),
    Left,
    Right,
}

fn line_to_vec(line: &str) -> Vec<Square> {
    return line
        .chars()
        .map(|c| match c {
            '.' => Square::Open,
            '#' => Square::Wall,
            _ => Square::Empty,
        })
        .collect();
}

const SPLIT_NEXT_TURN_RE: Regex = Regex::new(r"L|R").unwrap();

fn line_to_instructions(line: &str) -> Vec<Instructions> {
    if (line.len() == 0) {
        return Vec::new();
    } else if line[0] == 'R' {
        let mut instructions = line_to_instructions(l&ine[1..]);
        instructions.splice((0, 0), Instructions::Right);
        return instructions;
    } else if line[0] == 'L' {
        let mut instructions = line_to_instructions(l&ine[1..]);
        instructions.splice((0, 0), Instructions::Left);
        return instructions;
    } else {
        // Must be a number.
        let [number, remainder] = SPLIT_NEXT_TURN_RE.split(line);
        
    }

}

fn main() -> eyre::Result<()> {
    let mut map: Vec<Vec<Square>> = Vec::new();
    let instrucions: Vec<Instructions> = Vec::new();
    let sum = 0;
    for line in io::stdin().lock().lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            // End of map definition.
            instructions = line_to_instructions(line);
            break;
        }
        map.push(line_to_vec(line));
    }
    println!("Sum: {}", sum);
    Ok(())
}
