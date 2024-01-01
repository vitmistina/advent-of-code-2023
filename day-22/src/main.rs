use std::{collections::HashMap, fs};

mod chain_reaction;
mod counting;
mod falling;
mod parsing;
mod relations;
mod sorting;

fn main() {
    let input = &fs::read_to_string("input.txt").expect("File needs to be here!");
    let result = integrate(input);
    println!("Hello, world! {}", result.0);
    println!("Hello, world! {}", result.1);
}

struct Stack {
    terrain: Vec<Vec<usize>>,
    snapshot: Vec<Brick>,
    landed: Vec<Brick>,
    relations: HashMap<usize, Relation>,
}

#[derive(Debug, PartialEq, Clone)]
struct Relation {
    supports: Vec<usize>,
    stands_on: Vec<usize>,
}

#[derive(Debug, PartialEq, Clone)]
struct Brick {
    min: Coordinate,
    max: Coordinate,
    id: usize,
}

#[derive(Debug, PartialEq, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
    z: usize,
}

#[test]
fn computes_chains_with_memoization() {}

fn integrate(input: &str) -> (usize, usize) {
    let mut stack = Stack::parse(input);
    stack.sort_bricks();

    while stack.snapshot.is_empty() == false {
        match stack.fall_next_brick() {
            Ok(_) => continue,
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }

    stack.find_relations();

    (
        stack.count_safe_bricks(),
        stack.get_chain_lengths().iter().sum(),
    )
}

#[test]
fn integrates() {
    let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
    let result = integrate(input);
    assert_eq!(result.0, 5);
    assert_eq!(result.1, 7);
}
