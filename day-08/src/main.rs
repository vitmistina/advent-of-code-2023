use core::panic;
use num_integer::lcm;
use std::{collections::HashMap, fs};

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
    let desert_map = DesertMap::from(&data);
    println!("Hello, world! {}", desert_map.count_steps());
    println!("Ghost, world! {}", desert_map.count_ghost_steps_smartly());
}

#[derive(Debug, PartialEq)]
struct DesertMap {
    map: HashMap<String, (String, String)>,
    instructions: String,
}

impl DesertMap {
    fn from(data: &str) -> Self {
        let sections = data.split("\n\n").collect::<Vec<_>>();

        let mut map = HashMap::new();
        for line in sections.get(1).unwrap().lines() {
            map.insert(
                line[..3].to_string(),
                (line[7..10].to_string(), line[12..15].to_string()),
            );
        }
        let instructions = String::from(*sections.get(0).unwrap());

        Self { map, instructions }
    }

    fn count_steps(&self) -> u64 {
        let mut current_node_id = "AAA".to_string();
        let mut counter = 0;
        while current_node_id != "ZZZ" {
            for instruction in self.instructions.chars() {
                counter += 1;

                self.update_current_node(&mut current_node_id, instruction);

                if current_node_id == "ZZZ" {
                    break;
                }
            }
        }
        counter
    }

    fn update_current_node(&self, current_node_id: &mut String, instruction: char) {
        let current_node = self.map.get(&*current_node_id).unwrap();

        *current_node_id = match instruction {
            'L' => current_node.0.clone(),
            'R' => current_node.1.clone(),
            _ => panic!("Unexpected instruction"),
        };
    }

    fn count_ghost_steps_smartly(&self) -> u64 {
        let mut current_nodes = self
            .map
            .keys()
            .filter(|node_id| node_id.chars().last().unwrap() == 'A')
            .map(|node_id| node_id.into())
            .collect::<Vec<String>>();
        let mut cycle_times = current_nodes.iter().map(|_| 0u64).collect::<Vec<_>>();

        let mut counter = 0;
        let mut is_every_cycle_time_found = false;

        while is_every_cycle_time_found == false {
            for instruction in self.instructions.chars() {
                counter += 1;

                for (index, current_node_id) in current_nodes.iter_mut().enumerate() {
                    self.update_current_node(current_node_id, instruction);
                    if current_node_id.chars().last().unwrap() == 'Z'
                        && cycle_times.get(index).unwrap() == &0
                    {
                        let count = cycle_times.get_mut(index).unwrap();
                        *count = counter.clone();
                    };
                }

                is_every_cycle_time_found = cycle_times.iter().all(|time| time > &0);

                if is_every_cycle_time_found {
                    break;
                }
            }
        }

        println!("{:?}", cycle_times);

        cycle_times.iter().fold(1, |acc, cycle| lcm(acc, *cycle))
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parses_map() {
        let data = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let instructions = "LLR".to_string();
        let map = HashMap::from([
            ("AAA".to_string(), ("BBB".to_string(), "BBB".to_string())),
            ("BBB".to_string(), ("AAA".to_string(), "ZZZ".to_string())),
            ("ZZZ".to_string(), ("ZZZ".to_string(), "ZZZ".to_string())),
        ]);

        let expected_desert_map = DesertMap { map, instructions };

        assert_eq!(DesertMap::from(data), expected_desert_map);
    }

    #[test]
    fn find_ghost_path_smartly() {
        let instructions = "LR".to_string();
        let map = HashMap::from([
            ("11A".to_string(), ("11B".to_string(), "XXX".to_string())),
            ("11B".to_string(), ("XXX".to_string(), "11Z".to_string())),
            ("11Z".to_string(), ("11B".to_string(), "XXX".to_string())),
            ("22A".to_string(), ("22B".to_string(), "XXX".to_string())),
            ("22B".to_string(), ("22C".to_string(), "22C".to_string())),
            ("22C".to_string(), ("22Z".to_string(), "22Z".to_string())),
            ("22Z".to_string(), ("22B".to_string(), "22B".to_string())),
            ("XXX".to_string(), ("XXX".to_string(), "XXX".to_string())),
        ]);

        let desert_map = DesertMap { map, instructions };
        let step_count: u64 = desert_map.count_ghost_steps_smartly();
        assert_eq!(step_count, 6);
    }

    #[test]
    fn finds_path() {
        let instructions = "LLR".to_string();
        let map = HashMap::from([
            ("AAA".to_string(), ("BBB".to_string(), "BBB".to_string())),
            ("BBB".to_string(), ("AAA".to_string(), "ZZZ".to_string())),
            ("ZZZ".to_string(), ("ZZZ".to_string(), "ZZZ".to_string())),
        ]);

        let desert_map = DesertMap { map, instructions };
        let step_count: u64 = desert_map.count_steps();
        assert_eq!(step_count, 6);
    }
}
