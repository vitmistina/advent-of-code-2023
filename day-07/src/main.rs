use std::{cmp::Ordering, collections::HashMap, fs};

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
    let game = Game {
        value_map: get_values_map_simple(),
        count_rule: get_count_simple,
    };
    let winnings = game.calculate_total_winnings(&data);
    println!("Hello, world! {winnings}");

    let game = Game {
        value_map: get_values_map_joker(),
        count_rule: get_count_joker,
    };
    let winnings = game.calculate_total_winnings(&data);
    //251536526 too high
    //250857425 too low
    println!("Joker, world! {winnings}");
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Strength {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Hand {
    strenght: Strength,
    cards: String,
    multiplier: u16,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct SortableHand {
    strenght: Strength,
    card_strengths: Vec<u8>,
    cards: String,
    multiplier: u16,
}

struct Game {
    value_map: HashMap<char, u8>,
    count_rule: fn(&str) -> HashMap<char, u8>,
}

impl Game {
    fn calculate_total_winnings(&self, data: &str) -> u64 {
        let mut hands = self.parse_hands(data);

        sort_hands(&mut hands, &self.value_map);

        for hand in &hands {
            println!("{} {:?}", hand.cards, hand.strenght);
        }

        calculate_scores(&hands)
    }

    fn parse_hands(&self, data: &str) -> Vec<Hand> {
        data.lines()
            .map(|line| {
                let parts = line.split_whitespace().collect::<Vec<_>>();
                Hand {
                    strenght: calculate_strength(parts.get(0).unwrap(), self.count_rule),
                    cards: parts.get(0).unwrap().to_string(),
                    multiplier: parts.get(1).unwrap().parse::<u16>().unwrap(),
                }
            })
            .collect()
    }
}

fn calculate_scores(hands: &Vec<Hand>) -> u64 {
    hands.iter().enumerate().fold(0, |acc, (index, hand)| {
        acc + (hand.multiplier as u64 * (index as u64 + 1))
    })
}

fn get_values_map_simple() -> HashMap<char, u8> {
    HashMap::from([
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ])
}

fn get_values_map_joker() -> HashMap<char, u8> {
    HashMap::from([
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 1),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ])
}

fn sort_hands(hands: &mut Vec<Hand>, map: &HashMap<char, u8>) {
    hands.sort_unstable_by(|a, b| {
        let mut result = a.strenght.cmp(&b.strenght);
        for i in 0..5 {
            if result != Ordering::Equal {
                break;
            }

            let a_card = map.get(&a.cards.chars().nth(i).unwrap()).unwrap();
            let b_card = map.get(&b.cards.chars().nth(i).unwrap()).unwrap();

            result = b_card.cmp(&a_card);
        }
        result
    });
    hands.reverse();
}

fn calculate_strength(hand: &str, count_rule: fn(&str) -> HashMap<char, u8>) -> Strength {
    let card_count = count_rule(hand);

    if card_count.values().max().unwrap() == &5 {
        return Strength::FiveOfAKind;
    }

    if card_count.values().max().unwrap() == &4 {
        return Strength::FourOfAKind;
    }

    if card_count.values().max().unwrap() == &3 {
        if (card_count.values().collect::<Vec<&u8>>().contains(&&2)
            && card_count.get(&'J').unwrap() == &0)
            || card_count.values().filter(|card| card == &&3).count() == 2
        {
            return Strength::FullHouse;
        }
        return Strength::ThreeOfAKind;
    }

    let amount_of_pairs = card_count.values().filter(|card| card == &&2).count();
    if amount_of_pairs == 2 {
        return Strength::TwoPair;
    }

    if amount_of_pairs == 1 || amount_of_pairs > 2 {
        return Strength::OnePair;
    }

    Strength::HighCard
}

fn get_count_simple(hand: &str) -> HashMap<char, u8> {
    let mut card_count: HashMap<char, u8> = get_values_map_simple()
        .iter()
        .map(|card| (*card.0, 0u8))
        .collect();

    hand.chars().for_each(|card| {
        let count = card_count.get_mut(&card).unwrap();
        *count += 1;
    });
    card_count
}

fn get_count_joker(hand: &str) -> HashMap<char, u8> {
    let mut card_count: HashMap<char, u8> = get_values_map_simple()
        .iter()
        .map(|card| (*card.0, 0u8))
        .collect();

    hand.chars().for_each(|card| {
        let count = card_count.get_mut(&card).unwrap();
        *count += 1;
    });

    card_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_cards_with_joker() {
        let data = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
5JAQA 1
J25QT 2
AAJQQ 3";
        let game = Game {
            value_map: get_values_map_joker(),
            count_rule: get_count_joker,
        };

        let expected_hands = vec![
            Hand {
                cards: "32T3K".to_string(),
                multiplier: 765,
                strenght: Strength::OnePair,
            },
            Hand {
                cards: "T55J5".to_string(),
                multiplier: 684,
                strenght: Strength::FourOfAKind,
            },
            Hand {
                cards: "KK677".to_string(),
                multiplier: 28,
                strenght: Strength::TwoPair,
            },
            Hand {
                cards: "KTJJT".to_string(),
                multiplier: 220,
                strenght: Strength::FourOfAKind,
            },
            Hand {
                cards: "QQQJA".to_string(),
                multiplier: 483,
                strenght: Strength::FourOfAKind,
            },
            Hand {
                cards: "5JAQA".to_string(),
                multiplier: 1,
                strenght: Strength::ThreeOfAKind,
            },
            Hand {
                cards: "J25QT".to_string(),
                multiplier: 2,
                strenght: Strength::OnePair,
            },
            Hand {
                cards: "AAJQQ".to_string(),
                multiplier: 3,
                strenght: Strength::FullHouse,
            },
        ];

        assert_eq!(game.parse_hands(&data), expected_hands);
    }

    #[test]
    fn counts_jokers() {
        assert_eq!(get_count_joker("T55J5").get(&'5').unwrap(), &4);
        assert_eq!(get_count_joker("T55J5").get(&'T').unwrap(), &2);
        assert_eq!(get_count_joker("T55J5").get(&'J').unwrap(), &1);

        let card_count = get_count_joker("J25QT");
        let amount_of_pairs = card_count.values().filter(|card| card == &&2).count();
        assert_eq!(card_count.get(&'2').unwrap(), &2);
        assert_eq!(amount_of_pairs, 1);
    }

    #[test]
    fn integration_test() {
        let data = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let game = Game {
            value_map: get_values_map_simple(),
            count_rule: get_count_simple,
        };

        let result = game.calculate_total_winnings(&data);
        assert_eq!(result, 6440);
    }

    #[test]
    fn integration_joker() {
        let data = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let game = Game {
            value_map: get_values_map_joker(),
            count_rule: get_count_joker,
        };

        let result = game.calculate_total_winnings(&data);
        assert_eq!(result, 5905);
    }

    #[test]
    fn parses_cards() {
        let data = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let game = Game {
            value_map: get_values_map_simple(),
            count_rule: get_count_simple,
        };

        let expected_hands = vec![
            Hand {
                cards: "32T3K".to_string(),
                multiplier: 765,
                strenght: Strength::OnePair,
            },
            Hand {
                cards: "T55J5".to_string(),
                multiplier: 684,
                strenght: Strength::ThreeOfAKind,
            },
            Hand {
                cards: "KK677".to_string(),
                multiplier: 28,
                strenght: Strength::TwoPair,
            },
            Hand {
                cards: "KTJJT".to_string(),
                multiplier: 220,
                strenght: Strength::TwoPair,
            },
            Hand {
                cards: "QQQJA".to_string(),
                multiplier: 483,
                strenght: Strength::ThreeOfAKind,
            },
        ];

        assert_eq!(game.parse_hands(&data), expected_hands);
    }

    #[test]
    fn recognizes_correct_strenght() {
        assert_eq!(
            calculate_strength("99999", get_count_simple),
            Strength::FiveOfAKind
        );
        assert_eq!(
            calculate_strength("T55J5", get_count_simple),
            Strength::ThreeOfAKind
        );
        assert_eq!(
            calculate_strength("QQQJA", get_count_simple),
            Strength::ThreeOfAKind
        );
        assert_eq!(
            calculate_strength("QQQJJ", get_count_simple),
            Strength::FullHouse
        );
        assert_eq!(
            calculate_strength("KK677", get_count_simple),
            Strength::TwoPair
        );
        assert_eq!(
            calculate_strength("KTJJT", get_count_simple),
            Strength::TwoPair
        );
        assert_eq!(
            calculate_strength("59999", get_count_simple),
            Strength::FourOfAKind
        );
        assert_eq!(
            calculate_strength("9QATJ", get_count_simple),
            Strength::HighCard
        );
        assert_eq!(
            calculate_strength("32T3K", get_count_simple),
            Strength::OnePair
        );
    }

    #[test]
    fn orders_sortable_hands() {
        let mut hands = vec![
            SortableHand {
                cards: "59999".to_string(),
                multiplier: 483,
                strenght: Strength::FourOfAKind,
                card_strengths: vec![11, 4, 4, 4, 4],
            },
            SortableHand {
                cards: "JAAKK".to_string(),
                multiplier: 483,
                strenght: Strength::FourOfAKind,
                card_strengths: vec![15, 1, 1, 2, 2],
            },
            SortableHand {
                cards: "2AAAA".to_string(),
                multiplier: 483,
                strenght: Strength::FourOfAKind,
                card_strengths: vec![14, 1, 1, 1, 1],
            },
        ];

        let expected_hands = vec![
            SortableHand {
                cards: "JAAKK".to_string(),
                multiplier: 483,
                strenght: Strength::FourOfAKind,
                card_strengths: vec![15, 1, 1, 2, 2],
            },
            SortableHand {
                cards: "2AAAA".to_string(),
                multiplier: 483,
                strenght: Strength::FourOfAKind,
                card_strengths: vec![14, 1, 1, 1, 1],
            },
            SortableHand {
                cards: "59999".to_string(),
                multiplier: 483,
                strenght: Strength::FourOfAKind,
                card_strengths: vec![11, 4, 4, 4, 4],
            },
        ];

        hands.sort();
        hands.reverse();

        assert_eq!(hands, expected_hands);
    }
    #[test]
    fn orders_hands_correctly() {
        let mut initial_hands = vec![
            Hand {
                cards: "59999".to_string(),
                multiplier: 483,
                strenght: Strength::FourOfAKind,
            },
            Hand {
                cards: "2AAAA".to_string(),
                multiplier: 483,
                strenght: Strength::FourOfAKind,
            },
        ];

        let expected_hands = vec![
            Hand {
                cards: "2AAAA".to_string(),
                multiplier: 483,
                strenght: Strength::FourOfAKind,
            },
            Hand {
                cards: "59999".to_string(),
                multiplier: 483,
                strenght: Strength::FourOfAKind,
            },
        ];

        sort_hands(&mut initial_hands, &get_values_map_simple());

        assert_eq!(initial_hands, expected_hands);

        let mut initial_hands = vec![
            Hand {
                cards: "32T3K".to_string(),
                multiplier: 765,
                strenght: Strength::OnePair,
            },
            Hand {
                cards: "T55J5".to_string(),
                multiplier: 684,
                strenght: Strength::ThreeOfAKind,
            },
            Hand {
                cards: "KK677".to_string(),
                multiplier: 28,
                strenght: Strength::TwoPair,
            },
            Hand {
                cards: "KTJJT".to_string(),
                multiplier: 220,
                strenght: Strength::TwoPair,
            },
            Hand {
                cards: "QQQJA".to_string(),
                multiplier: 483,
                strenght: Strength::ThreeOfAKind,
            },
            Hand {
                cards: "59999".to_string(),
                multiplier: 483,
                strenght: Strength::FourOfAKind,
            },
            Hand {
                cards: "2AAAA".to_string(),
                multiplier: 483,
                strenght: Strength::FourOfAKind,
            },
        ];

        let expected_hands = vec![
            Hand {
                cards: "32T3K".to_string(),
                multiplier: 765,
                strenght: Strength::OnePair,
            },
            Hand {
                cards: "KTJJT".to_string(),
                multiplier: 220,
                strenght: Strength::TwoPair,
            },
            Hand {
                cards: "KK677".to_string(),
                multiplier: 28,
                strenght: Strength::TwoPair,
            },
            Hand {
                cards: "T55J5".to_string(),
                multiplier: 684,
                strenght: Strength::ThreeOfAKind,
            },
            Hand {
                cards: "QQQJA".to_string(),
                multiplier: 483,
                strenght: Strength::ThreeOfAKind,
            },
            Hand {
                cards: "2AAAA".to_string(),
                multiplier: 483,
                strenght: Strength::FourOfAKind,
            },
            Hand {
                cards: "59999".to_string(),
                multiplier: 483,
                strenght: Strength::FourOfAKind,
            },
        ];

        sort_hands(&mut initial_hands, &get_values_map_simple());

        assert_eq!(initial_hands, expected_hands);
    }

    #[test]
    fn calculates_scores_correctly() {
        let hands = vec![
            Hand {
                cards: "32T3K".to_string(),
                multiplier: 765,
                strenght: Strength::OnePair,
            },
            Hand {
                cards: "KTJJT".to_string(),
                multiplier: 220,
                strenght: Strength::TwoPair,
            },
            Hand {
                cards: "KK677".to_string(),
                multiplier: 28,
                strenght: Strength::TwoPair,
            },
            Hand {
                cards: "T55J5".to_string(),
                multiplier: 684,
                strenght: Strength::ThreeOfAKind,
            },
            Hand {
                cards: "QQQJA".to_string(),
                multiplier: 483,
                strenght: Strength::ThreeOfAKind,
            },
        ];

        assert_eq!(calculate_scores(&hands), 6440);
    }
}
