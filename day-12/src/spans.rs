use crate::Span;

pub(crate) fn find_spans(input: &str) -> Vec<Span> {
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
            output.push(Span {
                start: current_start.unwrap(),
                len: index + 1 - current_start.unwrap(),
            });
            current_start = None;
        }
    }

    output
}

#[test]
fn finds_hash_spans() {
    let input = "#";
    let result = find_spans(input);
    assert_eq!(result, vec![Span { start: 0, len: 1 }]);

    let input = "?#";
    let result = find_spans(input);
    assert_eq!(result, vec![Span { start: 1, len: 1 }]);

    let input = "?###????????";
    let result = find_spans(input);
    assert_eq!(result, vec![Span { start: 1, len: 3 }]);

    let input = "?#?#?#?";
    let result = find_spans(input);
    assert_eq!(
        result,
        vec![
            Span { start: 1, len: 1 },
            Span { start: 3, len: 1 },
            Span { start: 5, len: 1 }
        ]
    );
}
