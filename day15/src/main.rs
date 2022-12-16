use std::{io::{self, BufRead}, env};

use regex::Regex;

struct Sensor {
    position: (i32, i32),
    closest_beacon: (i32, i32),
}

fn line_to_sensor(line: &str) -> eyre::Result<Sensor> {
    let line_match: regex::Regex = Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$").unwrap();
    let mut match_result = line_match.captures_iter(line);
    if let Some(cap) = match_result.next() {
        let (s_x, s_y) = (cap[1].parse()?, cap[2].parse()?);
        let (b_x, b_y) = (cap[3].parse()?, cap[4].parse()?);
        Ok(Sensor{
            position: (s_x, s_y),
            closest_beacon: (b_x, b_y),
        })
    } else {
        Err(eyre::eyre!("Failed to match line: {line}"))
    }
}

fn main() -> eyre::Result<()> {
    let mut sensors = Vec::<Sensor>::new();
    // Read the sensors.
    for line in io::stdin().lock().lines() {
        let line = line?;
        sensors.push(line_to_sensor(&line)?)
    }
    // Read the row.
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(eyre::eyre!("Not enough arguments - provide the row id as an argument"));
    }
    let row: usize = args[1].parse()?;
    // TODO
    let impossible_count = 0;
    println!("Row {row} has {impossible_count} impossible positions");
    Ok(())
}
