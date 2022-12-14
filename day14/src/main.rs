use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Block {
    Air,
    Sand,
    Rock,
}

fn coordinate_to_tuple(coord: &str) -> eyre::Result<(usize, usize)> {
    let (x, y) = coord
        .split_once(',')
        .ok_or_else(|| eyre::eyre!("Unexpected coordinate: {coord}"))?;
    Ok((x.parse::<usize>()? - (500 - WORLD_WIDTH / 2), y.parse()?))
}

fn draw_rock_line(
    from: (usize, usize),
    to: (usize, usize),
    world: &mut [Vec<Block>],
) -> eyre::Result<()> {
    if from.0 == to.0 {
        // Vertical line, make sure it's top to bottom.
        let (from, to) = (from.min(to), from.max(to));
        for j in (from.1)..(to.1 + 1) {
            world[from.0][j] = Block::Rock;
        }
        Ok(())
    } else if from.1 == to.1 {
        // Horizontal line, make sure it's left to right.
        let (from, to) = (from.min(to), from.max(to));
        for column in world.iter_mut().take(to.0 + 1).skip(from.0) {
            column[from.1] = Block::Rock;
        }
        Ok(())
    } else {
        Err(eyre::eyre!(
            "Unexpected coordinates set: {from:?} to {to:?}"
        ))
    }
}

fn add_rock_from_line(line: &str, world: &mut [Vec<Block>]) -> eyre::Result<()> {
    let coordinates: eyre::Result<Vec<_>> = line.split(" -> ").map(coordinate_to_tuple).collect();
    let coordinates = coordinates?;
    if coordinates.len() == 1 {
        // Special case of a single rock
        world[coordinates[0].0][coordinates[1].1] = Block::Rock;
    }
    for i in 0..coordinates.len() - 1 {
        let from = coordinates[i];
        let to = coordinates[i + 1];
        draw_rock_line(from, to, world)?;
    }
    Ok(())
}

const WORLD_HEIGHT: usize = 400;
const WORLD_WIDTH: usize = 400;

fn add_sand(world: &mut [Vec<Block>]) -> eyre::Result<bool> {
    let mut x = WORLD_WIDTH / 2;
    let mut y = 0;
    if world[x][y] != Block::Air {
        return Ok(false);
    }
    while y < (WORLD_HEIGHT - 1) {
        if world[x][y + 1] == Block::Air {
            // Can fall down.
            y += 1;
        } else if world[x - 1][y + 1] == Block::Air {
            // Can fall down-left.
            x -= 1;
            y += 1;
        } else if world[x + 1][y + 1] == Block::Air {
            // Can fall down-right.
            x += 1;
            y += 1;
        } else {
            // I guess I live here now.
            world[x][y] = Block::Sand;
            return Ok(true);
        }
    }
    // Sand fell off the world.
    Err(eyre::eyre!("Sand fell off the world!"))
}

fn find_lowest_rock(world: &[Vec<Block>]) -> eyre::Result<usize> {
    let mut lowest = 0;
    for column in world {
        for (i, b) in column.iter().enumerate() {
            if *b == Block::Rock && i > lowest {
                lowest = i;
            }
        }
    }
    if lowest == 0 {
        Err(eyre::eyre!("No rock found!"))
    } else {
        Ok(lowest)
    }
}

fn main() -> eyre::Result<()> {
    let mut world = vec![vec![Block::Air; WORLD_HEIGHT]; WORLD_WIDTH];
    // Load the rock
    for line in io::stdin().lock().lines() {
        let line = line?;
        add_rock_from_line(&line, &mut world)?;
    }
    // Add the infinite rock layer.
    let lowest_rock = find_lowest_rock(&world)?;
    draw_rock_line(
        (0, lowest_rock + 2),
        (WORLD_WIDTH - 1, lowest_rock + 2),
        &mut world,
    )?;
    // Start pouring sand!
    let mut sand_added = 0;
    while add_sand(&mut world)? {
        sand_added += 1;
    }
    println!("Added {sand_added} sand");
    Ok(())
}
