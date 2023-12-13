fn find_recursively_with_space_needed(input: &str, numbers: &Vec<u8>) -> usize {
    let len = input.len();
    let spans = crate::spans::find_spans(input);
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
                            find_recursively_with_space_needed(
                                &input[potential_end_loc + 1..len],
                                &new_numbers,
                            )
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

#[test]
fn finds_combinations_recursively() {
    let input = "?";
    let numbers: Vec<u8> = vec![1];

    let result: usize = find_recursively_with_space_needed(input, &numbers);
    assert_eq!(result, 1);

    let input = "??";
    let numbers: Vec<u8> = vec![2];

    let result: usize = find_recursively_with_space_needed(input, &numbers);
    assert_eq!(result, 1);

    let input = "??";
    let numbers: Vec<u8> = vec![1];

    let result: usize = find_recursively_with_space_needed(input, &numbers);
    assert_eq!(result, 2);

    let input = "???";
    let numbers: Vec<u8> = vec![1, 1];

    let result: usize = find_recursively_with_space_needed(input, &numbers);
    assert_eq!(result, 1);

    let input = "????";
    let numbers: Vec<u8> = vec![1, 1];

    let result: usize = find_recursively_with_space_needed(input, &numbers);
    assert_eq!(result, 3);

    let input = "???????";
    let numbers: Vec<u8> = vec![2, 1];

    let result: usize = find_recursively_with_space_needed(input, &numbers);
    assert_eq!(result, 10);
}

#[test]
fn fails_for_long() {
    let input = "?";
    let numbers: Vec<u8> = vec![2];

    let result: usize = find_recursively_with_space_needed(input, &numbers);
    assert_eq!(result, 0);

    let input = "???";
    let numbers: Vec<u8> = vec![1, 2];

    let result: usize = find_recursively_with_space_needed(input, &numbers);
    assert_eq!(result, 0);
}
