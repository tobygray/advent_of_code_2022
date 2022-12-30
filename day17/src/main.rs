use std::{
    collections::VecDeque,
    io::{self, BufRead},
    time::Instant,
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
    y_offset: usize,
    last_used_row: Option<usize>,
}

//const BUFFER_SIZE: usize = 1000000000;
const BUFFER_SIZE: usize = 1000;
const BUFFER_DRAIN_SIZE: usize = BUFFER_SIZE / 10;

impl World {
    fn new() -> World {
        let mut ret = World {
            rows: VecDeque::new(),
            y_offset: 0,
            last_used_row: None,
        };
        ret.rows.resize(BUFFER_SIZE, 0);
        ret
    }

    fn reserve(&mut self, max_y: usize) {
        if (self.rows.len() + self.y_offset) >= max_y {
            return;
        }
        let _extra_rows = vec![0_u8; max_y - self.rows.len() - self.y_offset];
        self.rows.drain(0..BUFFER_DRAIN_SIZE);
        self.rows.resize(BUFFER_SIZE, 0);
        self.y_offset += BUFFER_DRAIN_SIZE;
    }

    fn store(&mut self, rock: &Piece, rock_x: i32, rock_y: usize) {
        match rock {
            Piece::Horizontal => {
                self.rows[rock_y - self.y_offset] |= 0b11110000 >> rock_x;
                self.set_last_used_row(rock_y);
            }
            Piece::Cross => {
                self.rows[rock_y + 2 - self.y_offset] |= 0b01000000 >> rock_x;
                self.rows[rock_y + 1 - self.y_offset] |= 0b11100000 >> rock_x;
                self.rows[rock_y - self.y_offset] |= 0b01000000 >> rock_x;
                self.set_last_used_row(rock_y + 2);
            }
            Piece::L => {
                self.rows[rock_y + 2 - self.y_offset] |= 0b00100000 >> rock_x;
                self.rows[rock_y + 1 - self.y_offset] |= 0b00100000 >> rock_x;
                self.rows[rock_y - self.y_offset] |= 0b11100000 >> rock_x;
                self.set_last_used_row(rock_y + 2);
            }
            Piece::Vertical => {
                self.rows[rock_y + 3 - self.y_offset] |= 0b10000000 >> rock_x;
                self.rows[rock_y + 2 - self.y_offset] |= 0b10000000 >> rock_x;
                self.rows[rock_y + 1 - self.y_offset] |= 0b10000000 >> rock_x;
                self.rows[rock_y - self.y_offset] |= 0b10000000 >> rock_x;
                self.set_last_used_row(rock_y + 3);
            }
            Piece::Square => {
                self.rows[rock_y + 1 - self.y_offset] |= 0b11000000 >> rock_x;
                self.rows[rock_y - self.y_offset] |= 0b11000000 >> rock_x;
                self.set_last_used_row(rock_y + 1);
            }
        };
    }

    fn set_last_used_row(&mut self, y: usize) {
        match self.last_used_row {
            Some(old) => {
                if y > old {
                    self.last_used_row = Some(y);
                }
            }
            None => {
                self.last_used_row = Some(y);
            }
        }
    }

    fn can_move_down(&self, rock: &Piece, rock_x: i32, rock_y: usize) -> bool {
        // Check world bounds.
        if rock_y == 0 {
            return false;
        }
        // Check other rocks.
        match rock {
            Piece::Horizontal => {
                self.rows[rock_y - 1 - self.y_offset] & (0b11110000 >> rock_x) == 0
            }
            Piece::Cross => {
                self.rows[rock_y + 1 - self.y_offset] & (0b01000000 >> rock_x) == 0
                    && self.rows[rock_y - self.y_offset] & (0b11100000 >> rock_x) == 0
                    && self.rows[rock_y - 1 - self.y_offset] & (0b01000000 >> rock_x) == 0
            }
            Piece::L => {
                self.rows[rock_y + 1 - self.y_offset] & (0b00100000 >> rock_x) == 0
                    && self.rows[rock_y - self.y_offset] & (0b00100000 >> rock_x) == 0
                    && self.rows[rock_y - 1 - self.y_offset] & (0b11100000 >> rock_x) == 0
            }
            Piece::Vertical => {
                self.rows[rock_y + 2 - self.y_offset] & (0b10000000 >> rock_x) == 0
                    && self.rows[rock_y + 1 - self.y_offset] & (0b10000000 >> rock_x) == 0
                    && self.rows[rock_y - self.y_offset] & (0b10000000 >> rock_x) == 0
                    && self.rows[rock_y - 1 - self.y_offset] & (0b10000000 >> rock_x) == 0
            }
            Piece::Square => {
                self.rows[rock_y - self.y_offset] & (0b11000000 >> rock_x) == 0
                    && self.rows[rock_y - 1 - self.y_offset] & (0b11000000 >> rock_x) == 0
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
            Piece::Horizontal => {
                self.rows[rock_y - self.y_offset] & (0b11110000 >> (rock_x - 1)) == 0
            }
            Piece::Cross => {
                self.rows[rock_y + 2 - self.y_offset] & (0b01000000 >> (rock_x - 1)) == 0
                    && self.rows[rock_y + 1 - self.y_offset] & (0b11100000 >> (rock_x - 1)) == 0
                    && self.rows[rock_y - self.y_offset] & (0b01000000 >> (rock_x - 1)) == 0
            }
            Piece::L => {
                self.rows[rock_y + 2 - self.y_offset] & (0b00100000 >> (rock_x - 1)) == 0
                    && self.rows[rock_y + 1 - self.y_offset] & (0b00100000 >> (rock_x - 1)) == 0
                    && self.rows[rock_y - self.y_offset] & (0b11100000 >> (rock_x - 1)) == 0
            }
            Piece::Vertical => {
                self.rows[rock_y + 3 - self.y_offset] & (0b10000000 >> (rock_x - 1)) == 0
                    && self.rows[rock_y + 2 - self.y_offset] & (0b10000000 >> (rock_x - 1)) == 0
                    && self.rows[rock_y + 1 - self.y_offset] & (0b10000000 >> (rock_x - 1)) == 0
                    && self.rows[rock_y - self.y_offset] & (0b10000000 >> (rock_x - 1)) == 0
            }
            Piece::Square => {
                self.rows[rock_y + 1 - self.y_offset] & (0b11000000 >> (rock_x - 1)) == 0
                    && self.rows[rock_y - self.y_offset] & (0b11000000 >> (rock_x - 1)) == 0
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
            Piece::Horizontal => {
                self.rows[rock_y - self.y_offset] & (0b11110000 >> (rock_x + 1)) == 0
            }
            Piece::Cross => {
                self.rows[rock_y + 2 - self.y_offset] & (0b01000000 >> (rock_x + 1)) == 0
                    && self.rows[rock_y + 1 - self.y_offset] & (0b11100000 >> (rock_x + 1)) == 0
                    && self.rows[rock_y - self.y_offset] & (0b01000000 >> (rock_x + 1)) == 0
            }
            Piece::L => {
                self.rows[rock_y + 2 - self.y_offset] & (0b00100000 >> (rock_x + 1)) == 0
                    && self.rows[rock_y + 1 - self.y_offset] & (0b00100000 >> (rock_x + 1)) == 0
                    && self.rows[rock_y - self.y_offset] & (0b11100000 >> (rock_x + 1)) == 0
            }
            Piece::Vertical => {
                self.rows[rock_y + 3 - self.y_offset] & (0b10000000 >> (rock_x + 1)) == 0
                    && self.rows[rock_y + 2 - self.y_offset] & (0b10000000 >> (rock_x + 1)) == 0
                    && self.rows[rock_y + 1 - self.y_offset] & (0b10000000 >> (rock_x + 1)) == 0
                    && self.rows[rock_y - self.y_offset] & (0b10000000 >> (rock_x + 1)) == 0
            }
            Piece::Square => {
                self.rows[rock_y + 1 - self.y_offset] & (0b11000000 >> (rock_x + 1)) == 0
                    && self.rows[rock_y - self.y_offset] & (0b11000000 >> (rock_x + 1)) == 0
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
    let start_instant = Instant::now();
    // Simulate rock falls
    let rock_count = 1000000000000;
    for (rock_number, rock) in rock_iter {
        if rock_number == 2022 {
            let max_row = world.last_used_row;
            println!("2022 highest row: {max_row:?}");
        }
        if (rock_number % 300000000) == 0 && rock_number != 0 {
            let now = Instant::now();
            let time_so_far = now - start_instant;
            let per_iteration = time_so_far.as_nanos() / (rock_number as u128);
            let remaining = (per_iteration * ((rock_count - rock_number) as u128)) / 1000000000;
            println!("At {rock_number}, runtime {time_so_far:?}, per_iteration {per_iteration:?}ns, remaining {remaining:?}s");
        }
        if rock_number == rock_count {
            break;
        }
        let mut rock_x = 2;
        let mut rock_y = world.last_used_row.map_or(3, |r| r + 4);
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
    let max_row = world.last_used_row;
    println!("Last filled row: {max_row:?}");
    Ok(())
}
