use std::collections::HashMap;

use crate::Parser;

pub(crate) struct WordParser {}

impl Parser for WordParser {
    fn parse_line(line: &str) -> u32 {
        let map = get_map();

        let unified_line = parse_words(line, map);

        let digits = gather_digits(unified_line);
        let mut last_first = String::new();
        last_first.push(digits.chars().nth(0).unwrap());
        last_first.push(digits.chars().nth_back(0).unwrap());
        last_first.parse::<u32>().unwrap()
    }

    fn sum_lines(input: &str) -> u32 {
        let mut values = Vec::new();
        for line in input.lines() {
            values.push(Self::parse_line(line));
        }
        values.iter().sum()
    }
}

fn gather_digits(unified_line: String) -> String {
    let mut digits = String::new();
    for char in unified_line.chars() {
        match char.to_digit(10) {
            Some(_) => digits.push(char),
            None => (),
        }
    }
    digits
}

fn parse_words(line: &str, map: HashMap<&str, &str>) -> String {
    let mut unified_line = String::new();

    for ch in line.chars() {
        unified_line.push(ch);
        for key in map.keys() {
            unified_line = unified_line.replace(key, map.get(key).unwrap())
        }
    }
    unified_line
}

fn get_map() -> HashMap<&'static str, &'static str> {
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("one", "1ne");
    map.insert("two", "2wo");
    map.insert("three", "3hree");
    map.insert("four", "4our");
    map.insert("five", "5ive");
    map.insert("six", "6ix");
    map.insert("seven", "7even");
    map.insert("eight", "8ight");
    map.insert("nine", "9ine");
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_lines() {
        let output = WordParser::parse_line("two1nine");
        assert_eq!(output, 29);
        let output = WordParser::parse_line("eightwothree");
        assert_eq!(output, 83);
        let output = WordParser::parse_line("abcone2threexyz");
        assert_eq!(output, 13);
        let output = WordParser::parse_line("xtwone3four");
        assert_eq!(output, 24);
        let output = WordParser::parse_line("4nineeightseven2");
        assert_eq!(output, 42);
        let output = WordParser::parse_line("zoneight234");
        assert_eq!(output, 14);
        let output = WordParser::parse_line("7pqrstsixteen");
        assert_eq!(output, 76);
        let output = WordParser::parse_line("oneoneeight");
        assert_eq!(output, 18);
    }

    #[test]
    fn sum() {
        let output = WordParser::sum_lines(
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen",
        );
        assert_eq!(output, 281);
    }
}
