use std::{
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

fn main() -> eyre::Result<()> {
    let mut sensors = Vec::<Sensor>::new();
    // Read the sensors.
    for line in io::stdin().lock().lines() {
        let line = line?;
        sensors.push(line_to_sensor(&line)?)
    }
    // Sort by the x value - might help with the later embedded X loop...
    sensors.sort_by_key(|s| s.position.0);
    // Read the row.
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(eyre::eyre!(
            "Not enough arguments - provide the max dimension as an argument"
        ));
    }
    let max: usize = args[1].parse()?;
    for j in 0..max + 1 {
        let mut i = 0;
        while i <= max as i32 {
            let x = i as i32;
            for sensor in &sensors {
                let distance_from_beacon = sensor.distance_from_beacon();
                let row_distance = sensor.position.1.abs_diff(j as i32);
                if row_distance > distance_from_beacon {
                    continue;
                }
                let x_width = (distance_from_beacon - row_distance) as i32;
                let start_impossible_x = sensor.position.0 - x_width;
                let end_impossible_x = sensor.position.0 + x_width;
                if i >= start_impossible_x && i <= end_impossible_x {
                    i = end_impossible_x + 1;
                    break;
                }
            }
            if x == i as i32 {
                println!("Frequency: {}", (x as u64 * 4000000) + (j as u64));
                return Ok(());
            }
        }
    }
    Ok(())
}
