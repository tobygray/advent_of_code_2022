use std::io::{self, BufRead};

#[derive(Debug)]
enum Piece {
    // ####
    Horizontal,
    // .#.
    // ###
    // .#.
    Cross,
    // ..#
    // ..#
    // ###
    L,
    // #
    // #
    // #
    // #
    Vertical,
    // ##
    // ##
    Square,
}

impl Piece {
    fn can_move_down(&self, world: &World, rock_x: i32, rock_y: usize) -> bool {
        // Check world bounds.
        if rock_y == 0 {
            return false;
        }
        // Check other rocks.
        match self {
            Piece::Horizontal => world.rows[rock_y - 1] & (0b11110000 >> rock_x) == 0,
            Piece::Cross => {
                world.rows[rock_y + 1] & (0b01000000 >> rock_x) == 0
                    && world.rows[rock_y] & (0b11100000 >> rock_x) == 0
                    && world.rows[rock_y - 1] & (0b01000000 >> rock_x) == 0
            }
            Piece::L => {
                world.rows[rock_y + 1] & (0b00100000 >> rock_x) == 0
                    && world.rows[rock_y] & (0b00100000 >> rock_x) == 0
                    && world.rows[rock_y - 1] & (0b11100000 >> rock_x) == 0
            }
            Piece::Vertical => {
                world.rows[rock_y + 2] & (0b10000000 >> rock_x) == 0
                    && world.rows[rock_y + 1] & (0b10000000 >> rock_x) == 0
                    && world.rows[rock_y] & (0b10000000 >> rock_x) == 0
                    && world.rows[rock_y - 1] & (0b10000000 >> rock_x) == 0
            }
            Piece::Square => {
                world.rows[rock_y] & (0b11000000 >> rock_x) == 0
                    && world.rows[rock_y - 1] & (0b11000000 >> rock_x) == 0
            }
        }
    }

    fn can_move_left(&self, world: &World, rock_x: i32, rock_y: usize) -> bool {
        // Check world bounds.
        if rock_x == 0 {
            return false;
        }
        // Check other rocks.
        match self {
            Piece::Horizontal => world.rows[rock_y] & (0b11110000 >> (rock_x - 1)) == 0,
            Piece::Cross => {
                world.rows[rock_y + 2] & (0b01000000 >> (rock_x - 1)) == 0
                    && world.rows[rock_y + 1] & (0b11100000 >> (rock_x - 1)) == 0
                    && world.rows[rock_y] & (0b01000000 >> (rock_x - 1)) == 0
            }
            Piece::L => {
                world.rows[rock_y + 2] & (0b00100000 >> (rock_x - 1)) == 0
                    && world.rows[rock_y + 1] & (0b00100000 >> (rock_x - 1)) == 0
                    && world.rows[rock_y] & (0b11100000 >> (rock_x - 1)) == 0
            }
            Piece::Vertical => {
                world.rows[rock_y + 3] & (0b10000000 >> (rock_x - 1)) == 0
                    && world.rows[rock_y + 2] & (0b10000000 >> (rock_x - 1)) == 0
                    && world.rows[rock_y + 1] & (0b10000000 >> (rock_x - 1)) == 0
                    && world.rows[rock_y] & (0b10000000 >> (rock_x - 1)) == 0
            }
            Piece::Square => {
                world.rows[rock_y + 1] & (0b11000000 >> (rock_x - 1)) == 0
                    && world.rows[rock_y] & (0b11000000 >> (rock_x - 1)) == 0
            }
        }
    }

    fn can_move_right(&self, world: &World, rock_x: i32, rock_y: usize) -> bool {
        // Check world bounds.
        match self {
            Piece::Horizontal => {
                if rock_x >= 3 {
                    return false;
                }
            }
            Piece::Cross => {
                if rock_x >= 4 {
                    return false;
                }
            }
            Piece::L => {
                if rock_x >= 4 {
                    return false;
                }
            }
            Piece::Vertical => {
                if rock_x >= 6 {
                    return false;
                }
            }
            Piece::Square => {
                if rock_x >= 5 {
                    return false;
                }
            }
        }
        // Check other rocks.
        match self {
            Piece::Horizontal => world.rows[rock_y] & (0b11110000 >> (rock_x + 1)) == 0,
            Piece::Cross => {
                world.rows[rock_y + 2] & (0b01000000 >> (rock_x + 1)) == 0
                    && world.rows[rock_y + 1] & (0b11100000 >> (rock_x + 1)) == 0
                    && world.rows[rock_y] & (0b01000000 >> (rock_x + 1)) == 0
            }
            Piece::L => {
                world.rows[rock_y + 2] & (0b00100000 >> (rock_x + 1)) == 0
                    && world.rows[rock_y + 1] & (0b00100000 >> (rock_x + 1)) == 0
                    && world.rows[rock_y] & (0b11100000 >> (rock_x + 1)) == 0
            }
            Piece::Vertical => {
                world.rows[rock_y + 3] & (0b10000000 >> (rock_x + 1)) == 0
                    && world.rows[rock_y + 2] & (0b10000000 >> (rock_x + 1)) == 0
                    && world.rows[rock_y + 1] & (0b10000000 >> (rock_x + 1)) == 0
                    && world.rows[rock_y] & (0b10000000 >> (rock_x + 1)) == 0
            }
            Piece::Square => {
                world.rows[rock_y + 1] & (0b11000000 >> (rock_x + 1)) == 0
                    && world.rows[rock_y] & (0b11000000 >> (rock_x + 1)) == 0
            }
        }
    }

    fn store(&self, world: &mut World, rock_x: i32, rock_y: usize) {
        match self {
            Piece::Horizontal => {
                world.rows[rock_y] |= 0b11110000 >> rock_x;
            }
            Piece::Cross => {
                world.rows[rock_y + 2] |= 0b01000000 >> rock_x;
                world.rows[rock_y + 1] |= 0b11100000 >> rock_x;
                world.rows[rock_y] |= 0b01000000 >> rock_x;
            }
            Piece::L => {
                world.rows[rock_y + 2] |= 0b00100000 >> rock_x;
                world.rows[rock_y + 1] |= 0b00100000 >> rock_x;
                world.rows[rock_y] |= 0b11100000 >> rock_x;
            }
            Piece::Vertical => {
                world.rows[rock_y + 3] |= 0b10000000 >> rock_x;

                world.rows[rock_y + 2] |= 0b10000000 >> rock_x;
                world.rows[rock_y + 1] |= 0b10000000 >> rock_x;
                world.rows[rock_y] |= 0b10000000 >> rock_x;
            }
            Piece::Square => {
                world.rows[rock_y + 1] |= 0b11000000 >> rock_x;
                world.rows[rock_y] |= 0b11000000 >> rock_x;
            }
        }
    }
}

enum JetDirection {
    Left,
    Right,
}

impl JetDirection {
    fn try_from(b: u8) -> eyre::Result<JetDirection> {
        if b == b'<' {
            Ok(JetDirection::Left)
        } else if b == b'>' {
            Ok(JetDirection::Right)
        } else {
            Err(eyre::eyre!("Unexpected byte: {b}"))
        }
    }
}

fn read_jet_pattern() -> eyre::Result<Vec<JetDirection>> {
    if let Some(line) = io::stdin().lock().lines().next() {
        let line = line?;
        let values: eyre::Result<Vec<_>> = line.bytes().map(JetDirection::try_from).collect();
        return values;
    }
    Err(eyre::eyre!("No input"))
}

struct World {
    rows: Vec<u8>,
}

impl World {
    fn new() -> World {
        World { rows: vec![] }
    }

    fn last_used_row(&self) -> eyre::Result<usize> {
        for (i, v) in self.rows.iter().rev().enumerate() {
            if *v != 0 {
                return Ok(self.rows.len() - i);
            }
        }
        Err(eyre::eyre!("No rows in use"))
    }

    fn reserve(&mut self, max_y: usize) {
        if self.rows.len() >= max_y {
            return;
        }
        let extra_rows = vec![0_u8; max_y - self.rows.len()];
        self.rows.extend(extra_rows);
    }

    fn print(&self) {
        println!();
        for row in self.rows.iter().rev() {
            print!("|");
            for idx in 0..7 {
                if row & (0b10000000 >> idx) == 0 {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            println!("|");
        }
        println!("+-------+");
    }
}

fn main() -> eyre::Result<()> {
    let jet_pattern = read_jet_pattern()?;
    let mut jet_iter = jet_pattern.iter().cycle();
    let rock_pattern = vec![
        Piece::Horizontal,
        Piece::Cross,
        Piece::L,
        Piece::Vertical,
        Piece::Square,
    ];
    let rock_iter = rock_pattern.iter().cycle().enumerate();
    let mut world = World::new();
    // Simulate rock falls
    let rock_count = 2022;
    for (rock_number, rock) in rock_iter {
        if rock_number == rock_count {
            break;
        }
        let mut rock_x = 2;
        let mut rock_y = world.last_used_row().map_or(3, |r| r + 3);
        world.reserve(rock_y + 4);
        loop {
            // Move the rock with the jet (if possible)
            match jet_iter.next().unwrap() {
                JetDirection::Left => {
                    if rock.can_move_left(&world, rock_x, rock_y) {
                        rock_x -= 1;
                    }
                }
                JetDirection::Right => {
                    if rock.can_move_right(&world, rock_x, rock_y) {
                        rock_x += 1;
                    }
                }
            }
            // Move the rock down.
            if !rock.can_move_down(&world, rock_x, rock_y) {
                break;
            }
            rock_y -= 1;
        }
        // Save the rock to the world.
        rock.store(&mut world, rock_x, rock_y);
    }
    let max_row = world.last_used_row()?;
    println!("Last filled row: {max_row}");
    Ok(())
}
