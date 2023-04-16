use std::io::{self, BufRead};

fn main() -> eyre::Result<()> {
    let _sum = 0;
    let mut numbers: Vec<(i32, usize)> = vec![];
    for (idx, line) in io::stdin().lock().lines().enumerate() {
        let line = line?;
        numbers.push((line.parse()?, idx));
    }
    let count = numbers.len();
    for original_idx in 0..count {
        let current_pos = numbers.iter().position(|&x| x.1 == original_idx).unwrap();
        let value = numbers[current_pos].0;
        if value == 0 {
            continue;
        }
        numbers.remove(current_pos);
        let mut new_pos_signed = current_pos as i32 + value;
        while new_pos_signed < 0 {
            new_pos_signed += numbers.len() as i32;
        }
        let new_pos = (new_pos_signed) as usize % (numbers.len());
        numbers.insert(new_pos, (value, original_idx));
    }
    let zero_idx = numbers.iter().position(|&x| x.0 == 0).unwrap();
    let idx1 = (zero_idx + 1000) % numbers.len();
    let idx2 = (zero_idx + 2000) % numbers.len();
    let idx3 = (zero_idx + 3000) % numbers.len();
    println!(
        "Sum: {}",
        numbers[idx1].0 + numbers[idx2].0 + numbers[idx3].0
    );
    Ok(())
}
