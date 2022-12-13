use std::{
    collections::VecDeque,
    io::{self, BufRead},
};

use serde_json::Value;

fn compare(lhs: &Value, rhs: &Value) -> eyre::Result<i32> {
    if let (Value::Number(l), Value::Number(r)) = (lhs, rhs) {
        Ok((l.as_i64().unwrap() - r.as_i64().unwrap()) as i32)
    } else if let (Value::Array(_l), Value::Number(_r)) = (lhs, rhs) {
        compare(lhs, &Value::Array(vec![rhs.to_owned()]))
    } else if let (Value::Number(_l), Value::Array(_r)) = (lhs, rhs) {
        compare(&Value::Array(vec![lhs.to_owned()]), rhs)
    } else if let (Value::Array(l), Value::Array(r)) = (lhs, rhs) {
        for i in 0..l.len().min(r.len()) {
            let result = compare(&l[i], &r[i])?;
            if result != 0 {
                return Ok(result);
            }
        }
        Ok(l.len() as i32 - r.len() as i32)
    } else {
        Err(eyre::eyre!("Unexpected values: {lhs:?} vs {rhs:?}"))
    }
}

fn main() -> eyre::Result<()> {
    let lines: Result<VecDeque<_>, _> = io::stdin().lock().lines().collect();
    let mut lines = lines?;
    let mut correct_sum = 0;
    let mut idx = 1;
    while lines.len() > 1 {
        let line1: Value = serde_json::from_str(&lines.pop_front().unwrap())?;
        let line2: Value = serde_json::from_str(&lines.pop_front().unwrap())?;
        // Discard the next blank line if present.
        lines.pop_front();
        if compare(&line1, &line2)? < 0 {
            correct_sum += idx;
        }
        idx += 1;
    }
    println!("Correct sum: {correct_sum}");
    Ok(())
}
