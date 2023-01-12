use std::io::{self, BufRead};

use regex::Regex;

#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore_ore_cost: u32,
    clay_ore_cost: u32,
    obsidian_ore_cost: u32,
    obsidian_clay_cost: u32,
    geode_ore_cost: u32,
    geode_obsidian_cost: u32,
}

fn line_to_blueprint(line: &str) -> eyre::Result<Blueprint> {
    let line_match: Regex = Regex::new(r"^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$").unwrap();
    let mut match_result = line_match.captures_iter(line);
    if let Some(cap) = match_result.next() {
        Ok(Blueprint {
            id: cap[1].parse()?,
            ore_ore_cost: cap[2].parse()?,
            clay_ore_cost: cap[3].parse()?,
            obsidian_ore_cost: cap[4].parse()?,
            obsidian_clay_cost: cap[5].parse()?,
            geode_ore_cost: cap[6].parse()?,
            geode_obsidian_cost: cap[7].parse()?,
        })
    } else {
        Err(eyre::eyre!("Failed to match line: {line}"))
    }
}

#[derive(Debug)]
struct Resources {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,
}

impl Resources {
    fn mine(&self, robots: &Robots) -> Resources {
        Resources {
            ore: self.ore + robots.ore,
            clay: self.clay + robots.clay,
            obsidian: self.obsidian + robots.obsidian,
            geodes: self.geodes + robots.geodes,
        }
    }
}

#[derive(Debug)]
struct Robots {
    ore: u32,
    max_ore: u32,
    clay: u32,
    max_clay: u32,
    obsidian: u32,
    max_obsidian: u32,
    geodes: u32,
}

impl Robots {
    fn add_ore(&self) -> Robots {
        Robots {
            ore: self.ore + 1,
            ..*self
        }
    }
    fn add_clay(&self) -> Robots {
        Robots {
            clay: self.clay + 1,
            ..*self
        }
    }
    fn add_obsidian(&self) -> Robots {
        Robots {
            obsidian: self.obsidian + 1,
            ..*self
        }
    }
    fn add_geode(&self) -> Robots {
        Robots {
            geodes: self.geodes + 1,
            ..*self
        }
    }
}

fn get_max_geodes(
    blueprint: &Blueprint,
    resources: &Resources,
    robots: &Robots,
    minutes_left: u32,
) -> u32 {
    //println!("Trying {blueprint:?} at {resources:?} with {robots:?} time left {minutes_left}");
    if minutes_left == 0 {
        return resources.geodes;
    }
    // Can always just build nothing this minute.
    let mut max_so_far = get_max_geodes(blueprint, &resources.mine(robots), robots, minutes_left - 1);
                            // Try building an ore robot if possible.
    if resources.ore >= blueprint.ore_ore_cost && robots.ore <= robots.max_ore {
        let mut resources = resources.mine(robots);
        resources.ore -= blueprint.ore_ore_cost;
        let max = get_max_geodes(blueprint, &resources, &robots.add_ore(), minutes_left - 1);
        if max > max_so_far {
            max_so_far = max;
        }
    }
    // Try building a clay robot if possible.
    if resources.ore >= blueprint.clay_ore_cost && robots.clay <= robots.max_clay {
        let mut resources = resources.mine(robots);
        resources.ore -= blueprint.clay_ore_cost;
        let max = get_max_geodes(blueprint, &resources, &robots.add_clay(), minutes_left - 1);
        if max > max_so_far {
            max_so_far = max;
        }
    }
    // Try building an obsidian robot if possible.
    if resources.ore >= blueprint.obsidian_ore_cost
        && resources.clay >= blueprint.obsidian_clay_cost
        && robots.obsidian <= robots.max_obsidian
    {
        let mut resources = resources.mine(robots);
        resources.ore -= blueprint.obsidian_ore_cost;
        resources.clay -= blueprint.obsidian_clay_cost;
        let max = get_max_geodes(
            blueprint,
            &resources,
            &robots.add_obsidian(),
            minutes_left - 1,
        );
        if max > max_so_far {
            max_so_far = max;
        }
    }
    // Try building a geode robot if possible.
    if resources.ore >= blueprint.geode_ore_cost
        && resources.obsidian >= blueprint.geode_obsidian_cost
    {
        let mut resources = resources.mine(robots);
        resources.ore -= blueprint.geode_ore_cost;
        resources.obsidian -= blueprint.geode_obsidian_cost;
        let max = get_max_geodes(blueprint, &resources, &robots.add_geode(), minutes_left - 1);
        if max > max_so_far {
            max_so_far = max;
        }
    }
    max_so_far
}

fn main() -> eyre::Result<()> {
    let mut blueprints = vec![];
    for line in io::stdin().lock().lines() {
        let line = line?;
        blueprints.push(line_to_blueprint(&line)?)
    }
    let mut sum = 0;
    for blueprint in blueprints {
        let max_geodes = get_max_geodes(
            &blueprint,
            &Resources {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geodes: 0,
            },
            &Robots {
                ore: 1,
                max_ore: blueprint
                    .ore_ore_cost
                    .max(blueprint.clay_ore_cost)
                    .max(blueprint.obsidian_ore_cost)
                    .max(blueprint.geode_ore_cost),
                clay: 0,
                max_clay: blueprint.obsidian_clay_cost,
                obsidian: 0,
                max_obsidian: blueprint.geode_obsidian_cost,
                geodes: 0,
            },
            24,
        );
        let quality = max_geodes * blueprint.id;
        sum += quality;
    }
    println!("Sum: {}", sum);
    Ok(())
}
