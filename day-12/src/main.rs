use std::{collections::HashMap, fs, ops::Sub};

use spans::find_spans;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = compute_variants(&input);
    print_variants(&input);
    //11955 too high
    //10133 too high
    //8921 too high
    //8375 too wrong
    //7792 right answer
    println!("Hello, world! {result}");
    let result = compute_unfolded_variants(&input);
    println!("Unfolded world! {result}");
}

mod space_needed_heuristic;
mod spans;

mod substring_method;

#[derive(Debug, PartialEq)]
struct Span {
    start: usize,
    len: usize,
}

#[derive(Eq, Hash, Debug, PartialEq)]
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

    fn unfold(&mut self) {
        self.springs = (0..4).fold(self.springs.clone(), |acc, _| acc + "?" + &self.springs);
        self.numbers = (0..4).fold(self.numbers.clone(), |acc, _| {
            [acc, self.numbers.clone()].concat()
        });
    }
}

fn compute_variants(input: &str) -> usize {
    let mut cache = HashMap::new();
    input
        .lines()
        .map(|line| JournalLine::parse_line(line))
        // .map(|j_line| find_recursively(&j_line.springs, &j_line.numbers, &mut cache).unwrap())
        .map(|j_line| {
            substring_method::find_recursively(&j_line.springs, &j_line.numbers, &mut cache)
                .unwrap()
        })
        .sum()
}

fn print_variants(input: &str) {
    let mut cache = HashMap::new();
    let lines = input
        .lines()
        .map(|line| JournalLine::parse_line(line))
        .map(|j_line| {
            // let result = find_recursively(&j_line.springs, &j_line.numbers, &mut cache).unwrap();
            let result =
                substring_method::find_recursively(&j_line.springs, &j_line.numbers, &mut cache)
                    .unwrap();
            format!("{} {}", result, j_line.springs)
        })
        .reduce(|acc, line| acc + &"\n" + &line)
        .unwrap();
    let _ = fs::write("faster.txt", lines).is_ok();
}

fn compute_unfolded_variants(input: &str) -> usize {
    let mut cache = HashMap::new();
    input
        .lines()
        .map(|line| {
            let mut j_line = JournalLine::parse_line(line);
            j_line.unfold();
            j_line
        })
        .map(|j_line| {
            substring_method::find_recursively(&j_line.springs, &j_line.numbers, &mut cache)
                .unwrap()
        })
        .sum()
}

fn find_recursively(
    input: &str,
    numbers: &Vec<u8>,
    cache: &mut HashMap<JournalLine, usize>,
) -> Option<usize> {
    let mut acc = 0;
    let j_line = JournalLine {
        springs: input.to_string(),
        numbers: numbers.clone(),
    };
    if let Some(cache_result) = cache.get(&j_line) {
        // println!("Cache hit: {:?}", j_line);
        return Some(*cache_result);
    };
    if let Some(first_question) = input.find('?') {
        let is_to_be_stopped = compute_span_match(&input, numbers, &first_question);
        match is_to_be_stopped {
            Some(result) => {
                if result == true {
                    cache.insert(j_line, 1);
                    return Some(1);
                }
            }
            None => {
                cache.insert(j_line, 0);
                return Some(0);
            }
        }
        let one_more_dot = input.replacen('?', ".", 1);
        acc += find_recursively(&one_more_dot, numbers, cache).unwrap();
        let one_more_hash = input.replacen('?', "#", 1);
        acc += find_recursively(&one_more_hash, numbers, cache).unwrap();
    } else {
        let spans = find_spans(input);
        if spans.len() == numbers.len()
            && spans
                .iter()
                .enumerate()
                .all(|(index, span)| numbers[index] == span.len as u8)
        {
            acc += 1;
        }
    }
    cache.insert(j_line, acc);

    Some(acc)
}

fn compute_span_match(input: &str, numbers: &[u8], first_question: &usize) -> Option<bool> {
    if numbers.iter().sum::<u8>() < input.chars().filter(|char| char == &'#').count() as u8 {
        // created too many hashes, solution won't exist
        return None;
    };

    let filled_input = &input[..*first_question];
    let spans = find_spans(filled_input);
    if numbers.len() < spans.len() {
        // created too many hash islands, solution won't exist
        return None;
    }

    let is_some_span_bigger = spans
        .iter()
        .enumerate()
        .any(|(index, span)| span.len as u8 > numbers[index]);
    if is_some_span_bigger == true {
        // created too long hash island, solution won't exist
        return None;
    }

    let is_exact_match = spans.len() == numbers.len()
        && spans
            .iter()
            .enumerate()
            .all(|(index, span)| span.len as u8 == numbers[index]);
    if is_exact_match == true {
        return Some(true);
    };

    Some(false)
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn expand() {
        let line = ".# 1";
        let mut j_line = JournalLine::parse_line(line);
        j_line.unfold();
        assert_eq!(j_line.springs, ".#?.#?.#?.#?.#");
        assert_eq!(j_line.numbers, vec![1, 1, 1, 1, 1]);

        let line = "???.### 1,1,3";
        let mut j_line = JournalLine::parse_line(line);
        j_line.unfold();
        assert_eq!(j_line.springs, "???.###????.###????.###????.###????.###");
        assert_eq!(
            j_line.numbers,
            vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3]
        );
    }

    #[test]
    fn break_me() {
        let line = "??#.???? 4";
        let j_line = JournalLine::parse_line(line);
        assert_eq!(
            find_recursively(&j_line.springs, &j_line.numbers, &mut HashMap::new()),
            Some(0)
        );

        let line = "#????? 4";
        let j_line = JournalLine::parse_line(line);
        assert_eq!(
            find_recursively(&j_line.springs, &j_line.numbers, &mut HashMap::new()),
            Some(1)
        );
    }

    #[test]
    fn real_examples() {
        let line = "?.?#?#??#?.?#????? 4,2,5";
        let j_line = JournalLine::parse_line(line);
        assert_eq!(
            find_recursively(&j_line.springs, &j_line.numbers, &mut HashMap::new()),
            Some(6)
        );

        let line = "?##??.#??#.???.# 4,4,2,1";
        let j_line = JournalLine::parse_line(line);
        assert_eq!(
            find_recursively(&j_line.springs, &j_line.numbers, &mut HashMap::new()),
            Some(4)
        );

        let line = "#?#?.##???.?.? 4,2,1,1";
        let j_line = JournalLine::parse_line(line);
        assert_eq!(
            find_recursively(&j_line.springs, &j_line.numbers, &mut HashMap::new()),
            Some(5)
        );
    }

    #[test]
    #[ignore]
    fn integration() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(compute_variants(input), 21);
        assert_eq!(compute_unfolded_variants(input), 525152);
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
        let result = find_recursively(input, &numbers, &mut HashMap::new());
        assert_eq!(result, Some(1));
    }

    #[test]
    fn four_six_five() {
        let input = "????.######..#####.";
        let numbers: Vec<u8> = vec![1, 6, 5];
        let result = find_recursively(input, &numbers, &mut HashMap::new());
        assert_eq!(result, Some(4));
    }
    #[test]
    fn three_two_one() {
        let input = "?###????????";
        let numbers: Vec<u8> = vec![3, 2, 1];
        let result = find_recursively(input, &numbers, &mut HashMap::new());
        assert_eq!(result, Some(10));
    }

    #[test]
    fn examples_from_code() {
        let input = "???.###";
        let numbers: Vec<u8> = vec![1, 1, 3];
        let result = find_recursively(input, &numbers, &mut HashMap::new());
        assert_eq!(result, Some(1));

        let input = ".??..??...?##.";
        let numbers: Vec<u8> = vec![1, 1, 3];
        let result = find_recursively(input, &numbers, &mut HashMap::new());
        assert_eq!(result, Some(4));

        let input = "?#?#?#?#?#?#?#?";
        let numbers: Vec<u8> = vec![1, 3, 1, 6];
        let result = find_recursively(input, &numbers, &mut HashMap::new());
        assert_eq!(result, Some(1));

        let input = "????.#...#...";
        let numbers: Vec<u8> = vec![4, 1, 1];
        let result = find_recursively(input, &numbers, &mut HashMap::new());
        assert_eq!(result, Some(1));
    }

    #[test]
    fn finds_combinations_recursively() {
        let input = "?";
        let numbers: Vec<u8> = vec![1];
        let result = find_recursively(input, &numbers, &mut HashMap::new());
        assert_eq!(result, Some(1));

        let input = "??";
        let numbers: Vec<u8> = vec![2];
        let result = find_recursively(input, &numbers, &mut HashMap::new());
        assert_eq!(result, Some(1));

        let input = "??";
        let numbers: Vec<u8> = vec![1];
        let result = find_recursively(input, &numbers, &mut HashMap::new());
        assert_eq!(result, Some(2));

        let input = ".??";
        let numbers: Vec<u8> = vec![1];
        let result = find_recursively(input, &numbers, &mut HashMap::new());
        assert_eq!(result, Some(2));

        let input = "?????";
        let numbers: Vec<u8> = vec![1, 1, 1];
        let result = find_recursively(input, &numbers, &mut HashMap::new());
        assert_eq!(result, Some(1));
    }

    #[test]
    fn negative_example() {
        // let input = "?";
        // let numbers: Vec<u8> = vec![2];
        // let result = find_recursively(input, &numbers, &mut HashMap::new());
        // assert_eq!(result, None);

        let input = "??";
        let numbers: Vec<u8> = vec![1, 1];
        let result = find_recursively(input, &numbers, &mut HashMap::new());
        assert_eq!(result, Some(0));
    }
}
