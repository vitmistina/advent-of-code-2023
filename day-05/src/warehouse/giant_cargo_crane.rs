use std::collections::HashMap;

use super::stack_of_crates::Stack;

#[derive(Debug, PartialEq)]
struct Instruction {
    repeats: u8,
    from: u8,
    to: u8,
}

pub(crate) struct Crane {
    instructions: Vec<Instruction>,
}

impl Crane {
    pub fn from(input: &str) -> Crane {
        let mut instructions = Vec::new();
        let lines: Vec<_> = input.lines().collect();
        for line in lines {
            instructions.push(Crane::parse_line(line))
        }
        Crane { instructions }
    }

    fn parse_line(line: &str) -> Instruction {
        let splitted: Vec<_> = line.split_whitespace().collect();
        let repeats = splitted[1].parse().expect("Wasn't u8");
        let from = splitted[3].parse().expect("Wasn't u8");
        let to = splitted[5].parse().expect("Wasn't u8");
        Instruction { repeats, from, to }
    }

    pub fn execute_instructions(&self, mut stacks: HashMap<u8, Stack>) -> HashMap<u8, Stack> {
        for instruction in &self.instructions {
            stacks = move_crate(&instruction, stacks)
        }
        stacks
    }
}

fn move_crate(instruction: &Instruction, mut stacks: HashMap<u8, Stack>) -> HashMap<u8, Stack> {
    let mut buffer = String::new();
    for _ in 0..instruction.repeats {
        let wooden_crate = stacks
            .get_mut(&instruction.from)
            .unwrap()
            .crates
            .pop()
            .unwrap();

        buffer.push(wooden_crate);
    }

    buffer = buffer.chars().rev().collect();

    stacks
        .get_mut(&instruction.to)
        .unwrap()
        .crates
        .push_str(&buffer);

    stacks
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn loads_4_instructions() {
        let input = "move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2";
        assert_eq!(4, Crane::from(input).instructions.len())
    }

    #[test]
    fn loads_right_instructions() {
        let input = "move 1 from 2 to 1";
        let expected = Vec::from([Instruction {
            repeats: 1,
            from: 2,
            to: 1,
        }]);
        assert_eq!(expected, Crane::from(input).instructions)
    }

    #[test]
    fn moves_crate() {
        let input_instruction = Instruction {
            repeats: 1,
            from: 1,
            to: 2,
        };
        let mut stacks = HashMap::new();
        stacks.insert(
            1,
            Stack {
                crates: "ABC".chars().collect(),
                id: 1,
            },
        );
        stacks.insert(
            2,
            Stack {
                crates: "XYZ".chars().collect(),
                id: 1,
            },
        );

        stacks = move_crate(&input_instruction, stacks);

        assert_eq!(2, stacks.get(&1).unwrap().crates.len());
        assert_eq!("AB".to_string(), stacks.get(&1).unwrap().crates);
        assert_eq!(4, stacks.get(&2).unwrap().crates.len());
        assert_eq!("XYZC".to_string(), stacks.get(&2).unwrap().crates);
    }
    #[test]
    fn repeats_instruction() {
        let instructions = vec![Instruction {
            repeats: 3,
            from: 1,
            to: 2,
        }];
        let mut stacks = HashMap::new();
        stacks.insert(
            1,
            Stack {
                crates: "ABC".chars().collect(),
                id: 1,
            },
        );
        stacks.insert(
            2,
            Stack {
                crates: "XYZ".chars().collect(),
                id: 1,
            },
        );

        let crane = Crane { instructions };

        stacks = crane.execute_instructions(stacks);

        assert_eq!(0, stacks.get(&1).unwrap().crates.len());
        assert_eq!("".to_string(), stacks.get(&1).unwrap().crates);
        assert_eq!(6, stacks.get(&2).unwrap().crates.len());
        assert_eq!("XYZABC".to_string(), stacks.get(&2).unwrap().crates);
    }
}
