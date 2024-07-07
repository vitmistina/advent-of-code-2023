use std::fs;

mod digit_parser;
use digit_parser::DigitParser;

mod word_parser;
use word_parser::WordParser;

fn main() {
    let input = fs::read_to_string("data.txt").unwrap();
    let sum_with_words = WordParser::sum_lines(&input);
    println!("Sum of all lines with Word Parser: {sum_with_words}");
    let sum = DigitParser::sum_lines(&input);
    println!("Sum of all lines with Digit Parser: {sum}");
}

trait Parser {
    fn parse_line(line: &str) -> u32;

    fn sum_lines(input: &str) -> u32;
}
