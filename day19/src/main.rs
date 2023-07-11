use std::io::{self, BufRead};

use regex::Regex;

#[derive(Debug)]
struct Blueprint {
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
    fn mine(&self, robots: &Robots, time: u32) -> Resources {
        Resources {
            ore: self.ore + (robots.ore * time),
            clay: self.clay + (robots.clay * time),
            obsidian: self.obsidian + (robots.obsidian * time),
            geodes: self.geodes + (robots.geodes * time),
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
    if minutes_left == 0 {
        return resources.geodes;
    }
    // Can always just build no more robots for the rest of the time.
    let mut max_so_far = resources.geodes + robots.geodes * minutes_left;
    // Try building an ore robot if useful.
    if robots.ore < robots.max_ore {
        if resources.ore >= blueprint.ore_ore_cost {
            // No need for more mining, can just build immediately.
            let mut resources = resources.mine(robots, 1);
            resources.ore -= blueprint.ore_ore_cost;
            let max = get_max_geodes(blueprint, &resources, &robots.add_ore(), minutes_left - 1);
            if max > max_so_far {
                max_so_far = max;
            }
        } else {
            let extra_ore_needed = blueprint.ore_ore_cost - resources.ore;
            let minutes_needed = (extra_ore_needed + robots.ore - 1) / robots.ore;
            if minutes_needed < minutes_left {
                let mut resources = resources.mine(robots, minutes_needed + 1);
                resources.ore -= blueprint.ore_ore_cost;
                let max = get_max_geodes(
                    blueprint,
                    &resources,
                    &robots.add_ore(),
                    minutes_left - minutes_needed - 1,
                );
                if max > max_so_far {
                    max_so_far = max;
                }
            }
        }
    }
    // Try building a clay robot if useful.
    if robots.clay < robots.max_clay {
        if resources.ore >= blueprint.clay_ore_cost {
            // No need for more mining, can just build immediately.
            let mut resources = resources.mine(robots, 1);
            resources.ore -= blueprint.clay_ore_cost;
            let max = get_max_geodes(blueprint, &resources, &robots.add_clay(), minutes_left - 1);
            if max > max_so_far {
                max_so_far = max;
            }
        } else {
            let extra_ore_needed = blueprint.clay_ore_cost - resources.ore;
            let minutes_needed = (extra_ore_needed + robots.ore - 1) / robots.ore;
            if minutes_needed < minutes_left {
                let mut resources = resources.mine(robots, minutes_needed + 1);
                resources.ore -= blueprint.clay_ore_cost;
                let max = get_max_geodes(
                    blueprint,
                    &resources,
                    &robots.add_clay(),
                    minutes_left - minutes_needed - 1,
                );
                if max > max_so_far {
                    max_so_far = max;
                }
            }
        }
    }
    // Try building an obsidian robot if possible and useful.
    if robots.clay > 0 && robots.obsidian < robots.max_obsidian {
        if resources.ore >= blueprint.obsidian_ore_cost
            && resources.clay >= blueprint.obsidian_clay_cost
        {
            // No need for more mining, can just build immediately.
            let mut resources = resources.mine(robots, 1);
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
        } else {
            let extra_ore_needed = if blueprint.obsidian_ore_cost > resources.ore {
                blueprint.obsidian_ore_cost - resources.ore
            } else {
                0
            };
            let extra_clay_needed = if blueprint.obsidian_clay_cost > resources.clay {
                blueprint.obsidian_clay_cost - resources.clay
            } else {
                0
            };
            let minutes_needed = ((extra_ore_needed + robots.ore - 1) / robots.ore)
                .max((extra_clay_needed + robots.clay - 1) / robots.clay);
            if minutes_needed < minutes_left {
                let mut resources = resources.mine(robots, minutes_needed + 1);
                resources.ore -= blueprint.obsidian_ore_cost;
                resources.clay -= blueprint.obsidian_clay_cost;
                let max = get_max_geodes(
                    blueprint,
                    &resources,
                    &robots.add_obsidian(),
                    minutes_left - minutes_needed - 1,
                );
                if max > max_so_far {
                    max_so_far = max;
                }
            }
        }
    }
    // Try building a geode robot if possible.
    if robots.obsidian > 0 {
        if resources.ore >= blueprint.geode_ore_cost
            && resources.obsidian >= blueprint.geode_obsidian_cost
        {
            // No need for more mining, can just build immediately.
            let mut resources = resources.mine(robots, 1);
            resources.ore -= blueprint.geode_ore_cost;
            resources.obsidian -= blueprint.geode_obsidian_cost;
            let max = get_max_geodes(blueprint, &resources, &robots.add_geode(), minutes_left - 1);
            if max > max_so_far {
                max_so_far = max;
            }
        } else {
            let extra_ore_needed = if blueprint.geode_ore_cost > resources.ore {
                blueprint.geode_ore_cost - resources.ore
            } else {
                0
            };
            let extra_obsidian_needed = if blueprint.geode_obsidian_cost > resources.obsidian {
                blueprint.geode_obsidian_cost - resources.obsidian
            } else {
                0
            };
            let minutes_needed = ((extra_ore_needed + robots.ore - 1) / robots.ore)
                .max((extra_obsidian_needed + robots.obsidian - 1) / robots.obsidian);
            if minutes_needed < minutes_left {
                let mut resources = resources.mine(robots, minutes_needed + 1);
                resources.ore -= blueprint.geode_ore_cost;
                resources.obsidian -= blueprint.geode_obsidian_cost;
                let max = get_max_geodes(
                    blueprint,
                    &resources,
                    &robots.add_geode(),
                    minutes_left - minutes_needed - 1,
                );
                if max > max_so_far {
                    max_so_far = max;
                }
            }
        }
    }
    max_so_far
}

fn main() -> eyre::Result<()> {
    let mut blueprints = vec![];
    for line in io::stdin().lock().lines() {
        let line = line?;
        blueprints.push(line_to_blueprint(&line)?);
        if blueprints.len() >= 3 {
            break;
        }
    }
    let mut product = 1;
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
            32,
        );
        product *= max_geodes;
    }
    println!("Product: {}", product);
    Ok(())
}
