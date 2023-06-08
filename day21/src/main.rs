use eyre::eyre;
use std::{
    collections::BTreeMap,
    io::{self, BufRead},
};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum Monkey {
    Value(f64),
    Plus(String, String),
    Minus(String, String),
    Multiply(String, String),
    Divide(String, String),
}

fn parse_op(op: &str) -> eyre::Result<Monkey> {
    let val = op.parse::<f64>();
    if let Ok(number) = val {
        return Ok(Monkey::Value(number));
    }
    let values: Vec<_> = op.split(' ').collect();
    if values.len() != 3 {
        return Err(eyre!("Unexpected op: {op}"));
    }
    match values[1] {
        "+" => Ok(Monkey::Plus(values[0].to_owned(), values[2].to_owned())),
        "-" => Ok(Monkey::Minus(values[0].to_owned(), values[2].to_owned())),
        "*" => Ok(Monkey::Multiply(values[0].to_owned(), values[2].to_owned())),
        "/" => Ok(Monkey::Divide(values[0].to_owned(), values[2].to_owned())),
        a => Err(eyre!("Unexpected operator: {a} for {op}")),
    }
}

fn parse_ln(line: &str) -> eyre::Result<(String, Monkey)> {
    let values: Vec<_> = line.split(": ").collect();
    if values.len() != 2 {
        return Err(eyre!("Unexpected line: {line}"));
    }
    let name = values[0];
    if name == "root" {
        let values: Vec<_> = values[1].split(' ').collect();
        Ok((
            name.to_owned(),
            Monkey::Minus(values[0].to_owned(), values[2].to_owned()),
        ))
    } else {
        let op = values[1];
        Ok((name.to_owned(), parse_op(op)?))
    }
}

fn main() -> eyre::Result<()> {
    let _sum = 0;
    let mut monkeys_with_values = BTreeMap::new();
    let mut monkeys_without_values = Vec::new();
    for line in io::stdin().lock().lines() {
        let line = line?;
        let (name, monkey) = parse_ln(&line)?;
        match monkey {
            Monkey::Value(v) => {
                monkeys_with_values.insert(name, v);
            }
            _ => {
                monkeys_without_values.push((name, monkey));
            }
        }
    }
    let mut lower_value = monkeys_with_values["humn"];
    let mut upper_value = lower_value;
    loop {
        upper_value *= 2.0;
        let value = get_value(monkeys_with_values.clone(), monkeys_without_values.clone(), upper_value )?;
        if value > 0.0 {
            lower_value = upper_value;
        } else {
            break;
        }
    }
    println!("Value is between {lower_value} and {upper_value}");
    loop {
        let guess = (lower_value + upper_value) / 2.0;
        let value = get_value(monkeys_with_values.clone(), monkeys_without_values.clone(), guess)?;
        println!("Monkey value: {value:?} for {guess} between {lower_value} and {upper_value}");
        if value == 0.0 {
            break;
        } else if value > 0.0 {
            lower_value = guess;
        } else {
            upper_value = guess;
        }
    }
    Ok(())
}
fn get_value(mut monkeys_with_values: BTreeMap<String, f64>, monkeys_without_values: Vec<(String, Monkey)>, humn: f64) -> eyre::Result<f64> {
    monkeys_with_values.insert("humn".to_owned(), humn);
    while !monkeys_with_values.contains_key("root") {
        for (name, monkey) in monkeys_without_values.iter() {
            match monkey {
                Monkey::Plus(lhs, rhs) => {
                    if !(monkeys_with_values.contains_key(lhs)
                        && monkeys_with_values.contains_key(rhs))
                    {
                        continue;
                    }
                    monkeys_with_values.insert(
                        name.to_owned(),
                        monkeys_with_values[lhs] + monkeys_with_values[rhs],
                    );
                }
                Monkey::Minus(lhs, rhs) => {
                    if !(monkeys_with_values.contains_key(lhs)
                        && monkeys_with_values.contains_key(rhs))
                    {
                        continue;
                    }
                    monkeys_with_values.insert(
                        name.to_owned(),
                        monkeys_with_values[lhs] - monkeys_with_values[rhs],
                    );
                }
                Monkey::Multiply(lhs, rhs) => {
                    if !(monkeys_with_values.contains_key(lhs)
                        && monkeys_with_values.contains_key(rhs))
                    {
                        continue;
                    }
                    monkeys_with_values.insert(
                        name.to_owned(),
                        monkeys_with_values[lhs] * monkeys_with_values[rhs],
                    );
                }
                Monkey::Divide(lhs, rhs) => {
                    if !(monkeys_with_values.contains_key(lhs)
                        && monkeys_with_values.contains_key(rhs))
                    {
                        continue;
                    }
                    monkeys_with_values.insert(
                        name.to_owned(),
                        monkeys_with_values[lhs] / monkeys_with_values[rhs],
                    );
                }
                _ => {
                    return Err(eyre!("Unexpected monkey with name {name}, {monkey:?}"));
                }
            }
        }
    }
    Ok(monkeys_with_values["root"])
}
