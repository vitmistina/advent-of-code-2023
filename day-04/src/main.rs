use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();

    let mut cards = data
        .lines()
        .map(|line| Card::parse_line(line))
        .collect::<Vec<Card>>();

    let score = cards.iter().map(|card| card.calculate_score()).sum::<u32>();
    println!("Total score is {score}");

    cards.iter_mut().for_each(|card| card.save_matches_count());

    println!("Total amount of cards is {}", sum_copy_scratchcards(&cards));
}

fn sum_copy_scratchcards(cards: &Vec<Card>) -> u32 {
    let mut map: HashMap<u8, u32> = HashMap::new();
    cards.iter().enumerate().for_each(|(index, _)| {
        map.insert(index as u8, 1);
    });

    for (index, card) in cards.iter().enumerate() {
        let card_count = map.get(&(index as u8)).unwrap();

        for _ in 0..*card_count {
            for j in 0..card.score.unwrap() {
                let target_index = index as u8 + j as u8 + 1;
                if let Some(card_amount) = map.get_mut(&target_index) {
                    *card_amount += 1;
                }
            }
        }
    }
    map.values().sum()
}

#[derive(PartialEq, Debug)]
struct Card {
    winning_numbers: HashSet<u32>,
    numbers_you_have: HashSet<u32>,
    score: Option<u32>,
}

impl Card {
    fn calculate_score(&self) -> u32 {
        let union = self.winning_numbers.intersection(&self.numbers_you_have);
        let length = union.count();
        match length {
            0 => 0,
            1 => 1,
            length => 2u32.pow((length - 1) as u32),
        }
    }

    fn save_matches_count(&mut self) {
        let union = self.winning_numbers.intersection(&self.numbers_you_have);
        let length = union.count();
        self.score = Some(length as u32);
    }

    fn parse_line(line: &str) -> Self {
        let header_and_content: Vec<&str> = line.split(":").collect();
        let winning_and_owned: Vec<&str> = header_and_content.get(1).unwrap().split("|").collect();
        let winning: Vec<u32> = winning_and_owned
            .get(0)
            .unwrap()
            .split_whitespace()
            .map(|num_string| num_string.parse::<u32>().unwrap())
            .collect();
        let owned: Vec<u32> = winning_and_owned
            .get(1)
            .unwrap()
            .split_whitespace()
            .map(|num_string| num_string.parse::<u32>().unwrap())
            .collect();
        Self {
            winning_numbers: HashSet::from_iter(winning),
            numbers_you_have: HashSet::from_iter(owned),
            score: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_copies_correctly() {
        let cards = vec![
            Card {
                winning_numbers: HashSet::new(),
                numbers_you_have: HashSet::new(),
                score: Some(4),
            },
            Card {
                winning_numbers: HashSet::new(),
                numbers_you_have: HashSet::new(),
                score: Some(2),
            },
            Card {
                winning_numbers: HashSet::new(),
                numbers_you_have: HashSet::new(),
                score: Some(2),
            },
            Card {
                winning_numbers: HashSet::new(),
                numbers_you_have: HashSet::new(),
                score: Some(1),
            },
            Card {
                winning_numbers: HashSet::new(),
                numbers_you_have: HashSet::new(),
                score: Some(0),
            },
            Card {
                winning_numbers: HashSet::new(),
                numbers_you_have: HashSet::new(),
                score: Some(0),
            },
        ];

        assert_eq!(sum_copy_scratchcards(&cards), 30)
    }

    #[test]
    fn parses_line() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

        let card = Card::parse_line(&line);

        assert_eq!(
            card,
            Card {
                winning_numbers: HashSet::from([41, 48, 83, 86, 17]),
                numbers_you_have: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]),
                score: None
            }
        );
    }

    #[test]
    fn calculates_score() {
        let card = Card {
            winning_numbers: HashSet::from([41, 48, 83, 86, 17]),
            numbers_you_have: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]),
            score: None,
        };

        let output = card.calculate_score();

        assert_eq!(output, 8);

        let card = Card {
            winning_numbers: HashSet::from([41, 92, 73, 84, 69]),
            numbers_you_have: HashSet::from([59, 84, 76, 51, 58, 5, 54, 83]),
            score: None,
        };

        let output = card.calculate_score();

        assert_eq!(output, 1);

        let card = Card {
            winning_numbers: HashSet::from([31, 18, 13, 56, 72]),
            numbers_you_have: HashSet::from([74, 77, 10, 23, 35, 67, 36, 11]),
            score: None,
        };

        let output = card.calculate_score();

        assert_eq!(output, 0);
    }
}
