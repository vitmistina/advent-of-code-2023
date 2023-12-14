use std::fs;

use mirrors::Mirrors;
use parsing::Parse;
use subslice::CreatesSubslice;
use transpose::Transposes;

mod mirrors;
mod parsing;
mod subslice;
mod transpose;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = find_result(&input);

    // 23746 too low
    // 35210 right
    println!("Hello, world! {result}");
}

#[derive(Debug, PartialEq, Clone)]
enum GroundType {
    Ash,
    Rocks,
}

#[derive(Debug, PartialEq)]
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
        for index in 0..pattern.data.len() {
            let slice = pattern.create_subslice(&index);
            if slice.check_mirrorness() == true {
                lines.push(index);
            }
        }
    }
    lines
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
