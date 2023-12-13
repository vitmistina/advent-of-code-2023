use std::{fs, ops::Sub};

use crate::spans::find_spans;

#[derive(Debug, PartialEq)]
struct Span {
    start: usize,
    len: usize,
}

#[derive(Debug, PartialEq)]
struct JournalLine {
    springs: String,
    numbers: Vec<u8>,
}
impl JournalLine {
    fn parse_line(line: &str) -> Self {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        Self {
            springs: parts.get(0).unwrap().to_string(),
            numbers: parts
                .get(1)
                .unwrap()
                .split(",")
                .map(|number| number.parse().unwrap())
                .collect(),
        }
    }
}

fn compute_variants(input: &str) -> usize {
    input
        .lines()
        .map(|line| JournalLine::parse_line(line))
        .map(|j_line| find_recursively(&j_line.springs, &j_line.numbers).unwrap())
        .sum()
}

fn find_recursively(input: &str, numbers: &Vec<u8>) -> Option<usize> {
    match numbers.get(0) {
        Some(number) => process_number(input, number, &numbers[1..].to_vec()),
        None => Some(0), //distributed all numbers already
    }
}

fn process_number(input: &str, number: &u8, rest_of_numbers: &Vec<u8>) -> Option<usize> {
    let trimmed = input.trim_matches('.');
    if *number as usize > trimmed.len() {
        return None;
    }
    let mut last_dot_separator = 0;
    let spans = find_spans(trimmed);
    let mut acc = 0;
    for (index, char) in trimmed.chars().enumerate() {
        if char == '.' {
            last_dot_separator = index;
            continue;
        }
        let potential_end_loc = index + (*number as usize);

        if let Some(after_end_char) = trimmed.chars().nth(potential_end_loc) {
            if after_end_char == '#' {
                continue;
            }
        }
        // let leftward_string = trimmed[last_dot_separator..index];
        if trimmed[last_dot_separator..index].contains('#') {
            continue;
        }
        if rest_of_numbers.len() == 0 && potential_end_loc <= trimmed.len() {
            if trimmed[index..potential_end_loc]
                .chars()
                .all(|char| char != '.')
            {
                acc += 1;
            }
        }
        if rest_of_numbers.len() > 0 && potential_end_loc < trimmed.len() {
            if trimmed[index..potential_end_loc]
                .chars()
                .any(|char| char == '.')
            {
                continue;
            }

            let current_string = &trimmed[index..potential_end_loc + 1];
            let next_string = &trimmed[potential_end_loc + 1..trimmed.len()];
            match find_recursively(next_string, rest_of_numbers) {
                Some(result) => {
                    acc += result;
                }
                None => {
                    break;
                }
            }
        }

        if let Some(span) = spans.get(0) {
            if span.len == (*number).into() && span.start == index {
                break;
            }
        }
    }
    Some(acc)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn break_me() {
        let line = "??#.???? 4";
        let j_line = JournalLine::parse_line(line);
        assert_eq!(find_recursively(&j_line.springs, &j_line.numbers), Some(1));

        let line = "#????? 4";
        let j_line = JournalLine::parse_line(line);
        assert_eq!(find_recursively(&j_line.springs, &j_line.numbers), Some(1));
    }

    #[test]
    fn real_examples() {
        let line = "?.?#?#??#?.?#????? 4,2,5";
        let j_line = JournalLine::parse_line(line);
        assert_eq!(find_recursively(&j_line.springs, &j_line.numbers), Some(6));

        let line = "?##??.#??#.???.# 4,4,2,1";
        let j_line = JournalLine::parse_line(line);
        assert_eq!(find_recursively(&j_line.springs, &j_line.numbers), Some(4));

        let line = "#?#?.##???.?.? 4,2,1,1";
        let j_line = JournalLine::parse_line(line);
        assert_eq!(find_recursively(&j_line.springs, &j_line.numbers), Some(5));
    }

    #[test]
    fn integration() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(compute_variants(input), 21);
    }

    #[test]
    fn parsing_test() {
        let input = "???.### 1,1,3";

        assert_eq!(
            JournalLine::parse_line(&input),
            JournalLine {
                springs: "???.###".to_string(),
                numbers: vec![1, 1, 3]
            }
        )
    }

    #[test]
    fn shouldnt_span_gap() {
        let input = "?.???";
        let numbers: Vec<u8> = vec![3];
        let result = find_recursively(input, &numbers);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn four_six_five() {
        let input = "????.######..#####.";
        let numbers: Vec<u8> = vec![1, 6, 5];
        let result = find_recursively(input, &numbers);
        assert_eq!(result, Some(4));
    }
    #[test]
    fn three_two_one() {
        let input = "?###????????";
        let numbers: Vec<u8> = vec![3, 2, 1];
        let result = find_recursively(input, &numbers);
        assert_eq!(result, Some(10));
    }

    #[test]
    fn examples_from_code() {
        let input = "???.###";
        let numbers: Vec<u8> = vec![1, 1, 3];
        let result = find_recursively(input, &numbers);
        assert_eq!(result, Some(1));

        let input = ".??..??...?##.";
        let numbers: Vec<u8> = vec![1, 1, 3];
        let result = find_recursively(input, &numbers);
        assert_eq!(result, Some(4));

        let input = "?#?#?#?#?#?#?#?";
        let numbers: Vec<u8> = vec![1, 3, 1, 6];
        let result = find_recursively(input, &numbers);
        assert_eq!(result, Some(1));

        let input = "????.#...#...";
        let numbers: Vec<u8> = vec![4, 1, 1];
        let result = find_recursively(input, &numbers);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn finds_combinations_recursively() {
        let input = "?";
        let numbers: Vec<u8> = vec![1];
        let result = find_recursively(input, &numbers);
        assert_eq!(result, Some(1));

        let input = "??";
        let numbers: Vec<u8> = vec![2];
        let result = find_recursively(input, &numbers);
        assert_eq!(result, Some(1));

        let input = "??";
        let numbers: Vec<u8> = vec![1];
        let result = find_recursively(input, &numbers);
        assert_eq!(result, Some(2));

        let input = ".??";
        let numbers: Vec<u8> = vec![1];
        let result = find_recursively(input, &numbers);
        assert_eq!(result, Some(2));

        let input = "?????";
        let numbers: Vec<u8> = vec![1, 1, 1];
        let result = find_recursively(input, &numbers);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn negative_example() {
        let input = "?";
        let numbers: Vec<u8> = vec![2];
        let result = find_recursively(input, &numbers);
        assert_eq!(result, None);

        let input = "??";
        let numbers: Vec<u8> = vec![1, 1];
        let result = find_recursively(input, &numbers);
        assert_eq!(result, Some(0));
    }
}
