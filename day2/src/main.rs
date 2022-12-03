use std::io::{self, BufRead};

use eyre::eyre;

#[derive(Debug, Copy, Clone)]
enum Throw {
    Rock,
    Paper,
    Scissors
}

fn line_to_throws(line: &str) -> eyre::Result<(Throw, Throw)> {
    let (them, us) = line.split_at(line.find(' ').ok_or_else(|| eyre!("Invalid input"))?);
    let them = match them {
        "A" => Throw::Rock,
        "B" => Throw::Paper,
        "C" => Throw::Scissors,
        _ => return Err(eyre!("Unexpected input for us {}", them)),
        };
    let us = match us {
        " X" => Throw::Rock,
        " Y" => Throw::Paper,
        " Z" => Throw::Scissors,
        _ => return Err(eyre!("Unexpected input for them {}", us)),
    };
    Ok((us, them))
}

#[derive(Debug, Copy, Clone)]
enum GameResult {
    Win,
    Loss,
    Draw
}

fn get_game_result(us: Throw, them: Throw) -> GameResult {
    match us {
        Throw::Paper => match them {
            Throw::Paper => GameResult::Draw,
            Throw::Rock => GameResult::Win,
            Throw::Scissors => GameResult::Loss,
        },
        Throw::Rock => match them {
            Throw::Paper => GameResult::Loss,
            Throw::Rock => GameResult::Draw,
            Throw::Scissors => GameResult::Win,
        },
        Throw::Scissors => match them {
            Throw::Paper => GameResult::Win,
            Throw::Rock => GameResult::Loss,
            Throw::Scissors => GameResult::Draw,
        }
    }
}


fn main() -> eyre::Result<()> {
    let mut total_score = 0;
    for line in io::stdin().lock().lines() {
        let line = line?;
        let _line = line.trim();
        let (us, them) = line_to_throws(&line)?;
        let result = get_game_result(us, them);
        let score = match result {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Loss => 0
        } + match us {
            Throw::Rock => 1,
            Throw::Paper => 2,
            Throw::Scissors => 3,
        };
        total_score += score
    }
    println!("Total score: {}", total_score);
    Ok(())
}
