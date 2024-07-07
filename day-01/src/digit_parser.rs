use crate::Parser;

pub(crate) struct DigitParser {}

impl Parser for DigitParser {
    fn parse_line(line: &str) -> u32 {
        let mut digits = String::new();
        for char in line.chars() {
            match char.to_digit(10) {
                Some(_) => digits.push(char),
                None => (),
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_lines() {
        let output = DigitParser::parse_line("1abc2");
        assert_eq!(output, 12);
        let output = DigitParser::parse_line("pqr3stu8vwx");
        assert_eq!(output, 38);
        let output = DigitParser::parse_line("a1b2c3d4e5f");
        assert_eq!(output, 15);
        let output = DigitParser::parse_line("treb7uchet");
        assert_eq!(output, 77);
    }

    #[test]
    fn sum() {
        let output = DigitParser::sum_lines(
            "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet",
        );
        assert_eq!(output, 142);
    }
}
