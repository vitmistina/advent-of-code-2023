use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = sum_comma_separated(&input);
    //515840 too low (pesky newline!)
    println!("Hello, world! {result}");

    let result = sum_focusing_powers(&input);
    println!("Hello, lenses! {result}");
}

#[derive(Debug, PartialEq)]
struct LensBox {
    lenses: Vec<Lens>,
}

#[derive(Debug, PartialEq)]
struct Lens {
    id: String,
    strenght: u8,
}

#[derive(Debug, PartialEq)]
enum Command {
    Upsert,
    Delete,
}

fn order_lenses_into_boxes(input: &str) -> HashMap<u8, LensBox> {
    let mut boxes: HashMap<u8, LensBox> = HashMap::new();
    input.split(",").for_each(|step| {
        if let Some(command_index) = step.find(|ch| ch == '-' || ch == '=') {
            let id = step[..command_index].to_string();
            let box_hash = ascii_hash(&id);

            let command = if step.chars().nth(command_index).unwrap() == '=' {
                Command::Upsert
            } else {
                Command::Delete
            };

            match boxes.get_mut(&box_hash) {
                Some(lens_box) => {
                    if command == Command::Delete {
                        if let Some(lens_index) =
                            lens_box.lenses.iter().position(|lens| lens.id == id)
                        {
                            lens_box.lenses.remove(lens_index);
                        };
                    }
                    if command == Command::Upsert {
                        let strenght = step[command_index + 1..].parse().unwrap();

                        if let Some(lens) = lens_box.lenses.iter_mut().find(|lens| lens.id == id) {
                            // update
                            lens.strenght = strenght;
                        } else {
                            // insert
                            lens_box.lenses.push(Lens { id, strenght });
                        };
                    }
                }
                None => {
                    if command == Command::Upsert {
                        let strenght = step[command_index + 1..].parse().unwrap();
                        boxes.insert(
                            box_hash,
                            LensBox {
                                lenses: vec![Lens { id, strenght }],
                            },
                        );
                    }
                }
            }
        };
    });
    boxes
}

fn sum_focusing_powers(input: &str) -> u64 {
    let boxes = order_lenses_into_boxes(input);
    boxes
        .iter()
        .map(|(k, lens_box)| -> u64 {
            lens_box
                .lenses
                .iter()
                .enumerate()
                .map(|(index, lens)| (*k as u64 + 1) * (1 + index as u64) * lens.strenght as u64)
                .sum::<u64>()
        })
        .sum()
}

fn sum_comma_separated(input: &str) -> u32 {
    input.split(",").map(|step| ascii_hash(step) as u32).sum()
}

fn ascii_hash(input: &str) -> u8 {
    input
        .chars()
        .fold(0, |acc, char| (acc + (char as u16)) * 17 % 256) as u8
}

#[test]
fn integration() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    assert_eq!(sum_focusing_powers(input), 145);
}

#[test]
fn removes_box() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-";
    assert_eq!(
        order_lenses_into_boxes(input),
        HashMap::from([
            (
                0,
                LensBox {
                    lenses: vec![
                        Lens {
                            id: String::from("rn"),
                            strenght: 1
                        },
                        Lens {
                            id: String::from("cm"),
                            strenght: 2
                        }
                    ],
                }
            ),
            (1, LensBox { lenses: Vec::new() })
        ])
    );
}

#[test]
fn creates_box() {
    let input = "rn=1";
    assert_eq!(
        order_lenses_into_boxes(input),
        HashMap::from([(
            0,
            LensBox {
                lenses: vec![Lens {
                    id: String::from("rn"),
                    strenght: 1
                }]
            }
        )])
    );
}

#[test]
fn hashes() {
    let input = "HASH";

    assert_eq!(ascii_hash(input), 52);
    assert_eq!(ascii_hash("rn"), 0);
    assert_eq!(ascii_hash("cm"), 0);
}

#[test]
fn part1_example() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    assert_eq!(sum_comma_separated(input), 1320);
}
