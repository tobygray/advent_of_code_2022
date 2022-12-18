use std::{
    collections::BTreeMap,
    io::{self, BufRead},
};

use regex::Regex;

struct ValveMap {
    links: BTreeMap<String, Vec<String>>,
    flow_rates: BTreeMap<String, u32>,
}

fn parse_input() -> eyre::Result<ValveMap> {
    let mut links = BTreeMap::<String, Vec<String>>::new();
    let mut flow_rates = BTreeMap::<String, u32>::new();
    let line_match: regex::Regex = Regex::new(
        r"^Valve (.+) has flow rate=(\d+); tunnel(?:s)? lead(?:s)? to valve(?:s)? (.*)$",
    )
    .unwrap();

    for line in io::stdin().lock().lines() {
        let line = line?;
        let mut match_result = line_match.captures_iter(&line);
        if let Some(cap) = match_result.next() {
            let valve_name = &cap[1];
            let flow_rate: u32 = cap[2].parse()?;
            let tunnels: Vec<_> = cap[3].split(", ").map(|s| s.to_owned()).collect();
            links.insert(valve_name.to_owned(), tunnels);
            flow_rates.insert(valve_name.to_owned(), flow_rate);
        } else {
            return Err(eyre::eyre!("Failed to match line: {line}"));
        }
    }
    Ok(ValveMap { links, flow_rates })
}

fn main() -> eyre::Result<()> {
    let _valve_map = parse_input()?;
    Ok(())
}
