use std::io::{self, BufRead};

use eyre::eyre;

#[derive(Debug, Copy, Clone)]
enum Throw {
    Rock,
    Paper,
    Scissors
}

fn line_to_throws(line: &str) -> eyre::Result<(GameResult, Throw)> {
    let (them, game_result) = line.split_at(line.find(' ').ok_or_else(|| eyre!("Invalid input"))?);
    let them = match them {
        "A" => Throw::Rock,
        "B" => Throw::Paper,
        "C" => Throw::Scissors,
        _ => return Err(eyre!("Unexpected input for them {}", them)),
        };
    let game_result = match game_result {
        " X" => GameResult::Loss,
        " Y" => GameResult::Draw,
        " Z" => GameResult::Win,
        _ => return Err(eyre!("Unexpected input for game result {}", game_result)),
    };
    Ok((game_result, them))
}

#[derive(Debug, Copy, Clone)]
enum GameResult {
    Win,
    Loss,
    Draw
}

fn get_our_move(game_result: GameResult, them: Throw) -> Throw {
    match them {
        Throw::Paper => match game_result {
            GameResult::Loss => Throw::Rock,
            GameResult::Draw => Throw::Paper,
            GameResult::Win => Throw::Scissors,
        },
        Throw::Rock => match game_result {
            GameResult::Loss => Throw::Scissors,
            GameResult::Draw => Throw::Rock,
            GameResult::Win => Throw::Paper,
        },
        Throw::Scissors => match game_result {
            GameResult::Loss => Throw::Paper,
            GameResult::Draw => Throw::Scissors,
            GameResult::Win => Throw::Rock,
        }
    }
}


fn main() -> eyre::Result<()> {
    let mut total_score = 0;
    for line in io::stdin().lock().lines() {
        let line = line?;
        let (result, them) = line_to_throws(&line)?;
        let us = get_our_move(result, them);
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
