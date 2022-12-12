use std::{
    io::{self, BufRead},
    usize::MAX,
};

struct Challenge {
    height_map_raw: Vec<i32>,
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
}

fn lines_to_height_map(lines: &[String]) -> eyre::Result<Challenge> {
    let width = lines[0].len();
    let height = lines.len();
    let mut height_map_raw = vec![0; width * height];
    let mut height_map_base: Vec<_> = height_map_raw.as_mut_slice().chunks_mut(width).collect();
    let height_map = height_map_base.as_mut_slice();

    let mut start = (0, 0);
    let mut end = (0, 0);
    for i in 0..height {
        let bytes = lines[i].as_bytes();
        for (j, value) in bytes.iter().enumerate() {
            if *value == b'S' {
                start = (i, j);
                height_map[i][j] = 'a' as i32;
            } else if *value == b'E' {
                end = (i, j);
                height_map[i][j] = 'z' as i32;
            } else {
                height_map[i][j] = *value as i32;
            }
        }
    }
    Ok(Challenge {
        height_map_raw,
        height,
        width,
        start,
        end,
    })
}

fn main() -> eyre::Result<()> {
    let lines: Result<Vec<_>, _> = io::stdin().lock().lines().collect();
    let lines = lines?;
    let Challenge {
        height_map_raw,
        height,
        width,
        start,
        end,
    } = lines_to_height_map(&lines)?;
    let height_map_base: Vec<_> = height_map_raw.as_slice().chunks(width).collect();
    let height_map = height_map_base.as_slice();
    let mut distances_raw: Vec<Option<usize>> = vec![None; height * width];
    let mut distances_base: Vec<_> = distances_raw.as_mut_slice().chunks_mut(width).collect();
    let distances = distances_base.as_mut_slice();

    distances[end.0][end.1] = Some(0);

    let mut updated = true;
    while updated {
        updated = false;
        for i in 0..height {
            for j in 0..width {
                if let Some(d) = distances[i][j] {
                    let current_height = height_map[i][j];
                    // Move here from up.
                    if i > 0
                        && height_map[i - 1][j] >= current_height - 1
                        && distances[i - 1][j].is_none()
                    {
                        distances[i - 1][j] = Some(d + 1);
                        updated = true;
                    }
                    // Move here from down.
                    if i < height - 1
                        && height_map[i + 1][j] >= current_height - 1
                        && distances[i + 1][j].is_none()
                    {
                        distances[i + 1][j] = Some(d + 1);
                        updated = true;
                    }
                    // Move here from left.
                    if j > 0
                        && height_map[i][j - 1] >= current_height - 1
                        && distances[i][j - 1].is_none()
                    {
                        distances[i][j - 1] = Some(d + 1);
                        updated = true;
                    }
                    // Move here from right.
                    if j < width - 1
                        && height_map[i][j + 1] >= current_height - 1
                        && distances[i][j + 1].is_none()
                    {
                        distances[i][j + 1] = Some(d + 1);
                        updated = true;
                    }
                }
            }
        }
    }

    let mut shortest_a = MAX;
    for i in 0..height {
        for j in 0..width {
            if let Some(d) = distances[i][j] {
                if shortest_a > d && height_map[i][j] == 'a' as i32 {
                    shortest_a = d;
                }
            }
        }
    }
    println!(
        "Minimum distance to start: {}",
        distances[start.0][start.1].unwrap()
    );
    println!("Minimum from a: {}", shortest_a);
    Ok(())
}
