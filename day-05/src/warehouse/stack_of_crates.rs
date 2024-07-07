use std::collections::HashMap;

#[derive(Debug, PartialEq)]

pub struct Stack {
    pub crates: String,
    pub id: u8,
}

impl Stack {
    pub fn from(input: &str) -> Stack {
        Stack {
            crates: input[1..].chars().collect(),
            id: input[0..1].parse().expect("Wasn't u8"),
        }
    }
}

pub fn load_stacks(input: &str) -> HashMap<u8, Stack> {
    let mut stacks = HashMap::new();
    let columns = divide_to_columns(input);
    for column in columns {
        let stack = Stack::from(&column);
        stacks.insert(stack.id, stack);
    }
    stacks
}

fn divide_to_columns(input: &str) -> Vec<String> {
    let mut columns: Vec<String> = Vec::new();
    let lines = input.lines();
    for line in lines.rev() {
        let chars = line.chars();
        for (char_index, char) in chars.enumerate() {
            if char_index % 4 == 1 && char.is_whitespace() == false {
                match columns.get_mut((char_index - 1) / 4) {
                    Some(string) => string.push(char),
                    None => columns.push(char.to_string()),
                }
            }
        }
    }
    columns
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parsing() {
        let input = "1WGVZ";
        let output = Stack::from(input);
        let expected = Stack {
            crates: "WGVZ".chars().collect(),
            id: 1,
        };
        assert_eq!(expected, output);
    }

    #[test]
    fn test_columns() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 ";
        let expected = Vec::from(["1ZN", "2MCD", "3P"]);
        let output = divide_to_columns(input);
        assert_eq!(3, output.len());
        assert_eq!(expected, output);
    }
}
