use std::fs;

use mirrors::Mirrors;
use parsing::Parse;
use smudges::FixesSmudges;
use subslice::CreatesSubslice;
use transpose::Transposes;

mod mirrors;
mod parsing;
mod smudges;
mod subslice;
mod transpose;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = find_result(&input);

    // 23746 too low
    // 35210 right
    assert_eq!(result, 35210);
    println!("Hello, world! {result}");

    let result = find_smudge_result(&input);

    //5209 too low
    //23680 too low
    //31794 too low
    println!("Hello, clean world! {result}");
}

#[derive(Debug, PartialEq, Clone)]
enum GroundType {
    Ash,
    Rocks,
}

impl GroundType {
    fn switch(&self) -> Self {
        match self {
            GroundType::Ash => GroundType::Rocks,
            GroundType::Rocks => GroundType::Ash,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Pattern {
    data: Vec<Vec<GroundType>>,
}

#[derive(Debug, PartialEq)]
struct Slice {
    data: Vec<Vec<GroundType>>,
}

fn find_result(input: &str) -> usize {
    let sections = Pattern::parse_sections(input);
    let rows = find_mirror_lines(&sections);
    let transposed = sections.iter().map(|pattern| pattern.transpose()).collect();
    let columns = find_mirror_lines(&transposed);
    let row_sum: usize = rows.iter().map(|row| row * 100).sum();
    let col_sum: usize = columns.iter().map(|col| col).sum();
    row_sum + col_sum
}

fn find_mirror_lines(sections: &Vec<Pattern>) -> Vec<usize> {
    let mut lines = Vec::new();
    for pattern in sections {
        for index in 1..pattern.data.len() {
            let slice = pattern.create_subslice(&index);
            if slice.check_mirrorness() == true {
                lines.push(index);
                break;
            }
        }
    }
    lines
}

fn find_smudge_result(input: &str) -> usize {
    let sections = Pattern::parse_sections(input);
    sections
        .iter()
        .map(|pattern| pattern.fix_smudge())
        .map(|(rows, cols)| rows * 100 + cols)
        .sum()
}

#[test]
fn finds_smudge_score() {
    let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    let result = find_smudge_result(input);
    assert_eq!(result, 400);
}

#[test]
fn finds_score() {
    let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    let result = find_result(input);
    assert_eq!(result, 405);
}
