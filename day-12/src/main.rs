fn main() {
    println!("Hello, world!");
}

fn find_recursively(input: &str, numbers: &Vec<u8>) -> usize {
    let len = input.len();
    let spans = find_spans(input);
    let mut new_numbers = numbers.clone();
    new_numbers.reverse();
    let result = match new_numbers.pop() {
        Some(number) => {
            let min_space_needed: usize = new_numbers
                .iter()
                .map(|new_number| *new_number as usize + 1)
                .sum();
            new_numbers.reverse();
            if let Some(usable_len) = len.checked_sub(min_space_needed) {
                //usable_len must be 0 or bigger
                input.chars().enumerate().fold(0, |acc, (index, char)| {
                    let potential_end_loc = index + (number as usize);

                    if new_numbers.len() == 0 {
                        acc + if potential_end_loc <= usable_len {
                            1
                        } else {
                            0
                        }
                    } else {
                        acc + if potential_end_loc < len {
                            find_recursively(&input[potential_end_loc + 1..len], &new_numbers)
                        } else {
                            0
                        }
                    }
                })
            } else {
                0
            }
        }
        None => 0,
    };
    result
}

fn find_spans(input: &str) -> Vec<(usize, usize)> {
    let mut output = Vec::new();
    let mut current_start = None;
    let mut iter = input.chars().enumerate().peekable();
    while let Some((index, char)) = iter.next() {
        if current_start.is_none() && char == '#' {
            current_start = Some(index);
        }

        let is_next_char_end_of_span = match iter.peek() {
            Some((_, next_char)) => {
                if next_char != &'#' {
                    true
                } else {
                    false
                }
            }
            None => true,
        };

        if current_start.is_some() && is_next_char_end_of_span == true {
            output.push((current_start.unwrap(), index + 1 - current_start.unwrap()));
            current_start = None;
        }
    }

    output
}

fn find_matching_span(input: &str, number: u8) {
    let spans = 
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn finds_hash_spans() {
        let input = "#";
        let result: Vec<(usize, usize)> = find_spans(input);
        assert_eq!(result, vec![(0, 1)]);

        let input = "?#";
        let result: Vec<(usize, usize)> = find_spans(input);
        assert_eq!(result, vec![(1, 1)]);

        let input = "?###????????";
        let result: Vec<(usize, usize)> = find_spans(input);
        assert_eq!(result, vec![(1, 3)]);

        let input = "?#?#?#?";
        let result: Vec<(usize, usize)> = find_spans(input);
        assert_eq!(result, vec![(1, 1), (3, 1), (5, 1)]);
    }

    #[test]
    fn finds_appropriate_substring() {
        let input = "#";
        let numbers: Vec<u8> = vec![1];
        let result: usize = find_recursively(input, &numbers);
        assert_eq!(result, 1);

        let input = "?#";
        let numbers: Vec<u8> = vec![1];
        let result: usize = find_recursively(input, &numbers);
        assert_eq!(result, 1);
    }

    #[test]
    fn finds_combinations_recursively() {
        let input = "?";
        let numbers: Vec<u8> = vec![1];

        let result: usize = find_recursively(input, &numbers);
        assert_eq!(result, 1);

        let input = "??";
        let numbers: Vec<u8> = vec![2];

        let result: usize = find_recursively(input, &numbers);
        assert_eq!(result, 1);

        let input = "??";
        let numbers: Vec<u8> = vec![1];

        let result: usize = find_recursively(input, &numbers);
        assert_eq!(result, 2);

        let input = "???";
        let numbers: Vec<u8> = vec![1, 1];

        let result: usize = find_recursively(input, &numbers);
        assert_eq!(result, 1);

        let input = "????";
        let numbers: Vec<u8> = vec![1, 1];

        let result: usize = find_recursively(input, &numbers);
        assert_eq!(result, 3);

        let input = "???????";
        let numbers: Vec<u8> = vec![2, 1];

        let result: usize = find_recursively(input, &numbers);
        assert_eq!(result, 10);
    }

    #[test]
    fn fails_for_long() {
        let input = "?";
        let numbers: Vec<u8> = vec![2];

        let result: usize = find_recursively(input, &numbers);
        assert_eq!(result, 0);

        let input = "???";
        let numbers: Vec<u8> = vec![1, 2];

        let result: usize = find_recursively(input, &numbers);
        assert_eq!(result, 0);
    }
}
