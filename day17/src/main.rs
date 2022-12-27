use std::{
    collections::VecDeque,
    io::{self, BufRead},
};

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
    rows: VecDeque<u8>,
}

impl World {
    fn new() -> World {
        World {
            rows: VecDeque::new(),
        }
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

    fn compress(&mut self, row: usize) {
        if self.rows[row] == 0b11111110 {
            println!("TETRIS at {row}!");
        }
    }

    fn store(&mut self, rock: &Piece, rock_x: i32, rock_y: usize) {
        match rock {
            Piece::Horizontal => {
                self.rows[rock_y] |= 0b11110000 >> rock_x;
                self.compress(rock_y);
            }
            Piece::Cross => {
                self.rows[rock_y + 2] |= 0b01000000 >> rock_x;
                self.rows[rock_y + 1] |= 0b11100000 >> rock_x;
                self.rows[rock_y] |= 0b01000000 >> rock_x;
                self.compress(rock_y + 2);
                self.compress(rock_y + 1);
                self.compress(rock_y);
            }
            Piece::L => {
                self.rows[rock_y + 2] |= 0b00100000 >> rock_x;
                self.rows[rock_y + 1] |= 0b00100000 >> rock_x;
                self.rows[rock_y] |= 0b11100000 >> rock_x;
                self.compress(rock_y + 2);
                self.compress(rock_y + 1);
                self.compress(rock_y);
            }
            Piece::Vertical => {
                self.rows[rock_y + 3] |= 0b10000000 >> rock_x;
                self.rows[rock_y + 2] |= 0b10000000 >> rock_x;
                self.rows[rock_y + 1] |= 0b10000000 >> rock_x;
                self.rows[rock_y] |= 0b10000000 >> rock_x;
                self.compress(rock_y + 3);
                self.compress(rock_y + 2);
                self.compress(rock_y + 1);
                self.compress(rock_y);
            }
            Piece::Square => {
                self.rows[rock_y + 1] |= 0b11000000 >> rock_x;
                self.rows[rock_y] |= 0b11000000 >> rock_x;
                self.compress(rock_y + 1);
                self.compress(rock_y);
            }
        };
    }

    fn can_move_down(&self, rock: &Piece, rock_x: i32, rock_y: usize) -> bool {
        // Check world bounds.
        if rock_y == 0 {
            return false;
        }
        // Check other rocks.
        match rock {
            Piece::Horizontal => self.rows[rock_y - 1] & (0b11110000 >> rock_x) == 0,
            Piece::Cross => {
                self.rows[rock_y + 1] & (0b01000000 >> rock_x) == 0
                    && self.rows[rock_y] & (0b11100000 >> rock_x) == 0
                    && self.rows[rock_y - 1] & (0b01000000 >> rock_x) == 0
            }
            Piece::L => {
                self.rows[rock_y + 1] & (0b00100000 >> rock_x) == 0
                    && self.rows[rock_y] & (0b00100000 >> rock_x) == 0
                    && self.rows[rock_y - 1] & (0b11100000 >> rock_x) == 0
            }
            Piece::Vertical => {
                self.rows[rock_y + 2] & (0b10000000 >> rock_x) == 0
                    && self.rows[rock_y + 1] & (0b10000000 >> rock_x) == 0
                    && self.rows[rock_y] & (0b10000000 >> rock_x) == 0
                    && self.rows[rock_y - 1] & (0b10000000 >> rock_x) == 0
            }
            Piece::Square => {
                self.rows[rock_y] & (0b11000000 >> rock_x) == 0
                    && self.rows[rock_y - 1] & (0b11000000 >> rock_x) == 0
            }
        }
    }

    fn can_move_left(&self, rock: &Piece, rock_x: i32, rock_y: usize) -> bool {
        // Check world bounds.
        if rock_x == 0 {
            return false;
        }
        // Check other rocks.
        match rock {
            Piece::Horizontal => self.rows[rock_y] & (0b11110000 >> (rock_x - 1)) == 0,
            Piece::Cross => {
                self.rows[rock_y + 2] & (0b01000000 >> (rock_x - 1)) == 0
                    && self.rows[rock_y + 1] & (0b11100000 >> (rock_x - 1)) == 0
                    && self.rows[rock_y] & (0b01000000 >> (rock_x - 1)) == 0
            }
            Piece::L => {
                self.rows[rock_y + 2] & (0b00100000 >> (rock_x - 1)) == 0
                    && self.rows[rock_y + 1] & (0b00100000 >> (rock_x - 1)) == 0
                    && self.rows[rock_y] & (0b11100000 >> (rock_x - 1)) == 0
            }
            Piece::Vertical => {
                self.rows[rock_y + 3] & (0b10000000 >> (rock_x - 1)) == 0
                    && self.rows[rock_y + 2] & (0b10000000 >> (rock_x - 1)) == 0
                    && self.rows[rock_y + 1] & (0b10000000 >> (rock_x - 1)) == 0
                    && self.rows[rock_y] & (0b10000000 >> (rock_x - 1)) == 0
            }
            Piece::Square => {
                self.rows[rock_y + 1] & (0b11000000 >> (rock_x - 1)) == 0
                    && self.rows[rock_y] & (0b11000000 >> (rock_x - 1)) == 0
            }
        }
    }

    fn can_move_right(&self, rock: &Piece, rock_x: i32, rock_y: usize) -> bool {
        // Check world bounds.
        match rock {
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
        match rock {
            Piece::Horizontal => self.rows[rock_y] & (0b11110000 >> (rock_x + 1)) == 0,
            Piece::Cross => {
                self.rows[rock_y + 2] & (0b01000000 >> (rock_x + 1)) == 0
                    && self.rows[rock_y + 1] & (0b11100000 >> (rock_x + 1)) == 0
                    && self.rows[rock_y] & (0b01000000 >> (rock_x + 1)) == 0
            }
            Piece::L => {
                self.rows[rock_y + 2] & (0b00100000 >> (rock_x + 1)) == 0
                    && self.rows[rock_y + 1] & (0b00100000 >> (rock_x + 1)) == 0
                    && self.rows[rock_y] & (0b11100000 >> (rock_x + 1)) == 0
            }
            Piece::Vertical => {
                self.rows[rock_y + 3] & (0b10000000 >> (rock_x + 1)) == 0
                    && self.rows[rock_y + 2] & (0b10000000 >> (rock_x + 1)) == 0
                    && self.rows[rock_y + 1] & (0b10000000 >> (rock_x + 1)) == 0
                    && self.rows[rock_y] & (0b10000000 >> (rock_x + 1)) == 0
            }
            Piece::Square => {
                self.rows[rock_y + 1] & (0b11000000 >> (rock_x + 1)) == 0
                    && self.rows[rock_y] & (0b11000000 >> (rock_x + 1)) == 0
            }
        }
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
    //let rock_count = 1000000000000;
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
                    if world.can_move_left(rock, rock_x, rock_y) {
                        rock_x -= 1;
                    }
                }
                JetDirection::Right => {
                    if world.can_move_right(rock, rock_x, rock_y) {
                        rock_x += 1;
                    }
                }
            }
            // Move the rock down.
            if !world.can_move_down(rock, rock_x, rock_y) {
                break;
            }
            rock_y -= 1;
        }
        // Save the rock to the world.
        world.store(rock, rock_x, rock_y);
    }
    let max_row = world.last_used_row()?;
    println!("Last filled row: {max_row}");
    Ok(())
}
