mod command;
mod parsing;

fn main() {
    println!("Hello, world!");
}

struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

#[derive(Debug, PartialEq)]
enum Operation {
    LessThan,
    GreaterThan,
}

#[derive(Debug, PartialEq)]
struct Condition {
    field: char,
    operation: Operation,
    value: u64,
}

#[derive(Debug, PartialEq)]
struct Command {
    condition: Option<Condition>,
    target: String,
}

#[test]
fn integration() {
    todo!()
}
