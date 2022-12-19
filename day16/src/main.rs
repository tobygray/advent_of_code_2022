use std::{
    collections::{BTreeMap},
    io::{self, BufRead},
};

use regex::Regex;

#[derive(Debug)]
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

fn map_routes(value_map: &ValveMap) -> eyre::Result<BTreeMap<String, BTreeMap<String, u32>>> {
    let mut ret = BTreeMap::new();
    for src_node in value_map.links.keys() {
        let mut route_lengths = BTreeMap::new();
        route_lengths.insert(src_node.to_owned(), 0);
        while route_lengths.len() < value_map.links.len() {
            let nodes_to_spread = route_lengths.clone();
            for (node, distance) in nodes_to_spread {
                let neighbours = value_map.links.get(&node).unwrap();
                for neighbour in neighbours {
                    match route_lengths.get(neighbour) {
                        Some(v) => {
                            if *v > distance + 1 {
                                route_lengths.insert(neighbour.to_owned(), distance + 1);
                            }
                        }
                        None => {
                            route_lengths.insert(neighbour.to_owned(), distance + 1);
                        }
                    }
                }
            }
        }
        ret.insert(src_node.to_owned(), route_lengths);
    }
    Ok(ret)
}

#[derive(Debug)]
struct WalkState<'a> {
    valve_map: &'a ValveMap,
    routes: &'a BTreeMap<String, BTreeMap<String, u32>>,
    current_valve: &'a str,
    time_limit: u32,
    current_flow: u32,
    current_rate: u32,
    max_rate: u32,
    max_flow_so_far: &'a mut u32,
    valves_to_open: &'a Vec<&'a String>,
}

fn walk_options(state: &mut WalkState) {
    if state.time_limit == 0 {
        // Ran out of time.
        if state.current_flow > *state.max_flow_so_far {
            *state.max_flow_so_far = state.current_flow;
        }
        return;
    }
    if state.valves_to_open.is_empty() {
        let final_rate = state.current_flow + (state.current_rate * state.time_limit);
        if final_rate > *state.max_flow_so_far {
            *state.max_flow_so_far = final_rate;
        }
        return;
    }
    if state.current_flow + (state.max_rate * state.time_limit) < *state.max_flow_so_far {
        // Not possible to better current max, so abort early.
        return;
    }
    for (i, valve_to_open) in state.valves_to_open.iter().enumerate() {
        if *valve_to_open == state.current_valve {
            // Open this valve, and tick on time by one step.
            let new_flow = state.current_flow + state.current_rate;
            let new_rate =
                state.current_rate + state.valve_map.flow_rates.get(state.current_valve).unwrap();
            let mut new_valves_to_open = state.valves_to_open.clone();
            new_valves_to_open.remove(i);
            walk_options(&mut WalkState {
                valve_map: state.valve_map,
                routes: state.routes,
                current_valve: state.current_valve,
                time_limit: state.time_limit - 1,
                current_flow: new_flow,
                max_flow_so_far: state.max_flow_so_far,
                current_rate: new_rate,
                max_rate: state.max_rate,
                valves_to_open: &new_valves_to_open,
            });
            return;
        }
        let route_length = state
            .routes
            .get(state.current_valve)
            .unwrap()
            .get(*valve_to_open)
            .unwrap();
        if *route_length >= state.time_limit {
            // Can't get to the valve and open it within time limit, so give up here.
            let final_rate = state.current_flow + (state.current_rate * state.time_limit);
            if final_rate > *state.max_flow_so_far {
                *state.max_flow_so_far = final_rate;
            }
        } else {
            walk_options(&mut WalkState {
                valve_map: state.valve_map,
                routes: state.routes,
                current_valve: valve_to_open,
                time_limit: state.time_limit - route_length,
                current_flow: state.current_flow + (state.current_rate * route_length),
                max_flow_so_far: state.max_flow_so_far,
                current_rate: state.current_rate,
                max_rate: state.max_rate,
                valves_to_open: state.valves_to_open,
            })
        }
    }
}

fn main() -> eyre::Result<()> {
    let valve_map = parse_input()?;
    let routes = map_routes(&valve_map)?;
    let time_limit = 26;
    let _max_flow_so_far = 0;
    let valves_to_open: Vec<String> = valve_map
        .flow_rates
        .iter()
        .filter(|(_, f)| **f > 0)
        .map(|(v, _)| v.to_owned())
        .collect();
    let valve_combinations: Vec<(Vec<_>, Vec<_>)> = (0..2usize.pow(valves_to_open.len() as u32)).map(|i| {
        let mut lhs = Vec::new();
        let mut rhs = Vec::new();
        for (t, v) in valves_to_open.iter().enumerate() {
            if (i >> t) %2 == 1 {
                lhs.push(v);
            } else {
                rhs.push(v);
            }
        }
        (lhs, rhs)
    }).collect();
    let mut max_flow_so_far = 0;
    for (lhs, rhs) in valve_combinations {
        let mut lhs_max_flow_so_far = 0;
        walk_options(&mut WalkState {
            valve_map: &valve_map,
            routes: &routes,
            current_valve: "AA",
            time_limit,
            current_flow: 0,
            current_rate: 0,
            max_rate: valve_map.flow_rates.values().sum(),
            max_flow_so_far: &mut lhs_max_flow_so_far,
            valves_to_open: &lhs,
    
        });
        let mut rhs_max_flow_so_far = 0;
        walk_options(&mut WalkState{
            valve_map: &valve_map,
            routes: &routes,
            current_valve: "AA",
            time_limit,
            current_flow: 0,
            current_rate: 0,
            max_rate: valve_map.flow_rates.values().sum(),
            max_flow_so_far: &mut rhs_max_flow_so_far,
            valves_to_open: &rhs,
        });
        let max_flow = lhs_max_flow_so_far + rhs_max_flow_so_far;
        if max_flow > max_flow_so_far {
            max_flow_so_far = max_flow;
        }
    }
    println!("Max flow rate: {max_flow_so_far}");
    Ok(())
}
