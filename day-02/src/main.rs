use std::fs;

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();

    let mut games = Vec::new();
    for line in data.lines() {
        games.push(Game::parse_game(line));
    }

    let possible_games = games.iter().filter(|game| game.is_possible);
    let sum_possible = possible_games.fold(0u16, |acc, game| acc + game.id as u16);
    println!("Sum of ids of possible games is {sum_possible}");

    let sum_powers = games.iter().fold(0u32, |acc, game| {
        acc + (game.red_max as u32 * game.green_max as u32 * game.blue_max as u32)
    });
    println!("Sum of powers is {sum_powers}");
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u8,
    red_max: u8,
    blue_max: u8,
    green_max: u8,
    is_possible: bool,
}

impl Game {
    fn parse_game(input: &str) -> Self {
        let parts: Vec<&str> = input.split(":").collect();

        let game_id = Self::get_game_id(&parts);
        let mut game = Self {
            id: game_id,
            red_max: 0,
            blue_max: 0,
            green_max: 0,
            is_possible: true,
        };
        game.get_color_amounts(parts);

        game
    }

    fn get_game_id(parts: &Vec<&str>) -> u8 {
        let game_string = parts.get(0).unwrap();
        let game_id = game_string
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u8>()
            .unwrap();
        game_id
    }

    fn get_color_amounts(&mut self, parts: Vec<&str>) {
        let rounds: Vec<&str> = parts.get(1).unwrap().split(";").collect();

        for round in rounds {
            let throws: Vec<&str> = round.split(",").collect();
            for throw in throws {
                let throw_parts: Vec<&str> = throw.split_whitespace().collect();

                let amount = throw_parts.get(0).unwrap().trim().parse::<u8>().unwrap();
                let color = *throw_parts.get(1).unwrap();

                match color {
                    "red" => {
                        if amount > self.red_max {
                            self.red_max = amount;
                            if amount > 12 {
                                self.is_possible = false;
                            }
                        }
                    }
                    "green" => {
                        if amount > self.green_max {
                            self.green_max = amount;
                            if amount > 13 {
                                self.is_possible = false;
                            }
                        }
                    }
                    "blue" => {
                        if amount > self.blue_max {
                            self.blue_max = amount;
                            if amount > 14 {
                                self.is_possible = false;
                            }
                        }
                    }
                    _ => panic!("Unknown color"),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_max() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        assert_eq!(
            Game::parse_game(input),
            Game {
                id: 1,
                red_max: 4,
                green_max: 2,
                blue_max: 6,
                is_possible: true
            }
        );
        let input = "Game 100: 6 green, 15 red, 12 blue; 9 red; 16 red; 17 red, 3 blue, 7 green";

        assert_eq!(
            Game::parse_game(input),
            Game {
                id: 100,
                red_max: 17,
                green_max: 7,
                blue_max: 12,
                is_possible: false
            }
        );
    }
}
