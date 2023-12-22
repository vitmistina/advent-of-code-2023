use super::*;

impl Command {
    fn parse(input: &str) -> Self {
        let mut parts = input.split(":");
        let (condition, target) = match parts.clone().count() {
            1 => (None, parts.next().unwrap().to_string()),
            2 => (
                Some(Condition::parse(parts.next().unwrap())),
                parts.next().unwrap().to_string(),
            ),
            _ => panic!(),
        };
        Self {
            condition: condition,
            target: target,
        }
    }
}

impl Condition {
    fn parse(input: &str) -> Self {
        let mut chars = input.chars();

        Self {
            field: chars.nth(0).unwrap(),
            operation: Operation::parse(chars.nth(0).unwrap()),
            value: chars.collect::<String>().parse().unwrap(),
        }
    }
}

impl Operation {
    fn parse(ch: char) -> Self {
        match ch {
            '>' => Self::GreaterThan,
            '<' => Self::LessThan,
            _ => panic!(),
        }
    }
}

#[test]
fn parses_condition() {
    let condition = "a<2006";
    assert_eq!(
        Condition::parse(condition),
        Condition {
            field: 'a',
            operation: Operation::LessThan,
            value: 2006
        }
    );
}

#[test]
fn parses_command() {
    let input = "a<2006:qkq";
    assert_eq!(
        Command::parse(input),
        Command {
            condition: Some(Condition {
                field: 'a',
                operation: Operation::LessThan,
                value: 2006
            }),
            target: String::from("qkq")
        }
    );

    let input = "rfg";
    assert_eq!(
        Command::parse(input),
        Command {
            condition: None,
            target: String::from("rfg")
        }
    );
}

#[test]
fn saves_command_to_map() {
    todo!()
}

#[test]
fn parses_part() {
    todo!()
}

#[test]
fn parses_full_input() {
    todo!()
}
