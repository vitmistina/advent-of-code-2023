use std::{collections::HashMap, fs};

use crate::{manual_processing::process_parts, range_processing::analyze_ranges};

mod command;
mod manual_processing;
mod parsing;
mod range_processing;

fn main() {
    let input = &fs::read_to_string("input.txt").unwrap();
    let result = process_parts(input);
    println!("Hello, world! {result}");

    let result = analyze_ranges(input);
    println!("Hello, ranges! {result}");
}

#[derive(Debug, PartialEq)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
    result: Option<CommandResult>,
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

#[derive(Debug, PartialEq, Clone)]
enum CommandResult {
    WorkflowSwitch(String),
    Next,
    Rejected,
    Accepted,
}

#[derive(Debug, PartialEq)]
struct Command {
    condition: Option<Condition>,
    target: String,
}

struct System {
    workflows: HashMap<String, Vec<Command>>,
    parts: Vec<Part>,
}

#[derive(Debug, PartialEq, Clone)]
struct Class {
    x: Bounds,
    m: Bounds,
    a: Bounds,
    s: Bounds,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Bounds {
    min: u64,
    max: u64,
}
