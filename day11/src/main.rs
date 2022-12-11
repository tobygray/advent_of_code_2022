use std::{io::{self, BufRead}, collections::VecDeque};

#[derive(Debug)]
enum MonkeyOperation {
    Multiply(i32),
    Add(i32),
    Square,
}

impl MonkeyOperation {
    fn apply(&self, value: i32) -> i32 {
        match self {
            MonkeyOperation::Add(a) => value + a,
            MonkeyOperation::Multiply(m) => value * m,
            MonkeyOperation::Square => value * value,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<i32>,
    operation: MonkeyOperation,
    modulus: i32,
    if_true: i32,
    if_false: i32,
    total_item_count: i32,
}

const STARTING_ITEMS_PREFIX: &str = "  Starting items: ";
const SQUARE_OP: &str = "  Operation: new = old * old";
const ADD_PREFIX: &str = "  Operation: new = old + ";
const MUL_PREFIX: &str = "  Operation: new = old * ";
const TEST_PREFIX: &str = "  Test: divisible by ";
const TEST_TRUE_PREFIX: &str = "    If true: throw to monkey ";
const TEST_FALSE_PREFIX: &str = "    If false: throw to monkey ";

fn lines_to_monkey(lines: &mut Vec<String>) -> eyre::Result<Option<Monkey>> {
    if lines.is_empty() {
        return Ok(None);
    }
    if lines.len() < 6 {
        return Err(eyre::eyre!(
            "Not enough lines for a monkey remain: {:?}",
            lines
        ));
    }
    let items: VecDeque<_> = lines[1][STARTING_ITEMS_PREFIX.len()..]
        .split(", ")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let operation = if lines[2] == SQUARE_OP {
        MonkeyOperation::Square
    } else if lines[2].starts_with(ADD_PREFIX) {
        MonkeyOperation::Add(lines[2][ADD_PREFIX.len()..].parse()?)
    } else if lines[2].starts_with(MUL_PREFIX) {
        MonkeyOperation::Multiply(lines[2][MUL_PREFIX.len()..].parse()?)
    } else {
        return Err(eyre::eyre!("Unexpected operation: {}", lines[2]));
    };
    let modulus = lines[3][TEST_PREFIX.len()..].parse()?;
    let if_true = lines[4][TEST_TRUE_PREFIX.len()..].parse()?;
    let if_false = lines[5][TEST_FALSE_PREFIX.len()..].parse()?;
    lines.drain(0..6);
    if !lines.is_empty() && lines[0].trim().is_empty() {
        lines.remove(0);
    }
    Ok(Some(Monkey {
        items,
        operation,
        modulus,
        if_true,
        if_false,
        total_item_count: 0,
    }))
}

fn main() -> eyre::Result<()> {
    let mut monkeys = Vec::<Monkey>::new();
    let lines: Result<Vec<_>, _> = io::stdin().lock().lines().collect();
    let mut lines = lines?;
    while !lines.is_empty() {
        if let Some(m) = lines_to_monkey(&mut lines)? {
            println!("Monkey: {:?}", m);
            monkeys.push(m);
        }
    }
    println!("Read {} monkeys", monkeys.len());
    for r in 0..20 {
        println!("Round {}", r + 1);
        for idx in 0..monkeys.len() {
            while !monkeys[idx].items.is_empty() {
                monkeys[idx].total_item_count += 1;
                let item = monkeys[idx].items.pop_front().unwrap();
                let item = monkeys[idx].operation.apply(item) / 3;
                let next_idx = if item % monkeys[idx].modulus == 0 {
                    monkeys[idx].if_true
                } else {
                    monkeys[idx].if_false
                } as usize;
                monkeys[next_idx].items.push_back(item);
            }
        }
    }
    let mut total_item_counts:Vec<_> = monkeys.iter().map(|m| m.total_item_count).collect();
    total_item_counts.sort();
    total_item_counts.reverse();
    println!("Top two monkeys {} and {}: {}", total_item_counts[0], total_item_counts[1], total_item_counts[0] * total_item_counts[1]);
    Ok(())
}
