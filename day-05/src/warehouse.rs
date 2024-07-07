mod giant_cargo_crane;
mod splitter;
mod stack_of_crates;
use std::collections::HashMap;

use giant_cargo_crane::Crane;
use splitter::Inits;

use self::stack_of_crates::{load_stacks, Stack};

pub struct Warehouse {
    crane: Crane,
    stacks: HashMap<u8, Stack>,
}

impl Warehouse {
    pub fn from(input: &str) -> Warehouse {
        let inits = Inits::from(input);
        let stacks = load_stacks(&inits.stacks_map);
        Warehouse {
            crane: Crane::from(&inits.crane_instructions),
            stacks,
        }
    }

    pub fn start_crane(mut self) -> Warehouse {
        self.stacks = self.crane.execute_instructions(self.stacks);
        self
    }

    pub fn read_top(self) -> String {
        let mut top_crates = String::new();
        for id in 1..(self.stacks.len() + 1) {
            top_crates.push(
                self.stacks
                    .get(&u8::try_from(id).unwrap())
                    .unwrap()
                    .crates
                    .chars()
                    .last()
                    .unwrap(),
            );
        }
        top_crates
    }
}
