use std::{
    cmp::Ordering,
    collections::VecDeque,
    io::{self, BufRead},
};

use serde_json::Value;

fn compare(lhs: &Value, rhs: &Value) -> eyre::Result<Ordering> {
    if let (Value::Number(l), Value::Number(r)) = (lhs, rhs) {
        Ok(l.as_i64().unwrap().cmp(&r.as_i64().unwrap()))
    } else if let (Value::Array(_l), Value::Number(_r)) = (lhs, rhs) {
        compare(lhs, &Value::Array(vec![rhs.to_owned()]))
    } else if let (Value::Number(_l), Value::Array(_r)) = (lhs, rhs) {
        compare(&Value::Array(vec![lhs.to_owned()]), rhs)
    } else if let (Value::Array(l), Value::Array(r)) = (lhs, rhs) {
        for i in 0..l.len().min(r.len()) {
            let result = compare(&l[i], &r[i])?;
            if result != Ordering::Equal {
                return Ok(result);
            }
        }
        Ok(l.len().cmp(&r.len()))
    } else {
        Err(eyre::eyre!("Unexpected values: {lhs:?} vs {rhs:?}"))
    }
}

fn main() -> eyre::Result<()> {
    let lines: Result<VecDeque<_>, _> = io::stdin().lock().lines().collect();
    let mut lines = lines?;
    let mut correct_sum = 0;
    let mut idx = 1;
    let mut all_values: Vec<Value> = Vec::new();
    while lines.len() > 1 {
        let line1: Value = serde_json::from_str(&lines.pop_front().unwrap())?;
        let line2: Value = serde_json::from_str(&lines.pop_front().unwrap())?;
        // Discard the next blank line if present.
        lines.pop_front();
        if compare(&line1, &line2)? == Ordering::Less {
            correct_sum += idx;
        }
        idx += 1;
        all_values.push(line1);
        all_values.push(line2);
    }
    let special_packets = vec![
        Value::Array(vec![Value::Array(vec![Value::Number(2.try_into()?)])]),
        Value::Array(vec![Value::Array(vec![Value::Number(6.try_into()?)])]),
    ];
    special_packets
        .iter()
        .for_each(|p| all_values.push(p.to_owned()));
    all_values.sort_by(|a, b| compare(a, b).unwrap());

    println!("Correct sum for out of order packets: {correct_sum}");
    let special_packet_product: usize = special_packets
        .iter()
        .map(|p| {
            all_values
                .binary_search_by(|f| compare(f, p).unwrap())
                .unwrap()
                + 1
        })
        .product();
    println!("Special packet location product: {special_packet_product}");
    Ok(())
}
