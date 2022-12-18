use std::{
    collections::BTreeSet,
    env,
    io::{self, BufRead},
};

use regex::Regex;

#[derive(Debug)]
struct Sensor {
    position: (i32, i32),
    closest_beacon: (i32, i32),
}

impl Sensor {
    fn distance_from_beacon(&self) -> u32 {
        self.position.0.abs_diff(self.closest_beacon.0)
            + self.position.1.abs_diff(self.closest_beacon.1)
    }
}

fn line_to_sensor(line: &str) -> eyre::Result<Sensor> {
    let line_match: regex::Regex =
        Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$")
            .unwrap();
    let mut match_result = line_match.captures_iter(line);
    if let Some(cap) = match_result.next() {
        let (s_x, s_y) = (cap[1].parse()?, cap[2].parse()?);
        let (b_x, b_y) = (cap[3].parse()?, cap[4].parse()?);
        Ok(Sensor {
            position: (s_x, s_y),
            closest_beacon: (b_x, b_y),
        })
    } else {
        Err(eyre::eyre!("Failed to match line: {line}"))
    }
}

fn impossible_positions(sensor: &Sensor, row: i32) -> eyre::Result<Option<Vec<i32>>> {
    let height_from_beacon = sensor.position.1.abs_diff(row);
    let total_distance = sensor.distance_from_beacon();
    if height_from_beacon >= total_distance {
        return Ok(None);
    }
    let row_distance = total_distance - height_from_beacon;
    let center = sensor.position.0;
    let start = center - row_distance as i32;
    let end = center + row_distance as i32;
    Ok(Some((start..end).collect()))
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
        return Err(eyre::eyre!(
            "Not enough arguments - provide the row id as an argument"
        ));
    }
    let row: i32 = args[1].parse()?;
    let impossible_position_rows: eyre::Result<Vec<_>> = sensors
        .iter()
        .map(|s| impossible_positions(s, row))
        .collect();
    let binding: Vec<_> = impossible_position_rows?
        .iter()
        .filter_map(|x| x.clone())
        .collect();
    let impossible_position_rows: Vec<BTreeSet<_>> =
        binding.iter().map(|v| v.iter().collect()).collect();
    let impossible_position_rows: BTreeSet<_> = impossible_position_rows
        .iter()
        .fold(BTreeSet::new(), |a, b| a.union(b).cloned().collect());
    let impossible_count = impossible_position_rows.len();
    println!("Row {row} has {impossible_count} impossible positions");
    Ok(())
}
