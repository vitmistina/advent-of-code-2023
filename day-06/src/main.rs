use std::{collections::HashSet, fs};

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
    let count = count_start(&data);
    println!("Number of characters: {}", count);
}

fn count_start(input: &str) -> u16 {
    const NUM_CHARS: usize = 14;
    let mut counter = 0;
    let mut buffer = String::new();
    for char in input.chars() {
        buffer.push(char);
        counter += 1;
        if buffer.len() > NUM_CHARS {
            buffer = String::from(&buffer[1..]);
            let myset: HashSet<char> = buffer.chars().collect();
            if myset.len() == NUM_CHARS {
                return counter;
            }
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_start() {
        assert_eq!(19, count_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(23, count_start("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(23, count_start("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(29, count_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(26, count_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
}
