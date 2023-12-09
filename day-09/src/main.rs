use std::fs;

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();

    let results = data
        .lines()
        .map(|line| parse_line(line))
        .map(|sequence| predict(&sequence, forwards_prediction))
        .sum::<i64>();

    println!("Sum of predictions {results}");

    let results = data
        .lines()
        .map(|line| parse_line(line))
        .map(|sequence| predict(&sequence, backwards_prediction))
        .sum::<i64>();

    println!("Sum of backwards predictions {results}");
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|num_string| num_string.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
}

fn predict(sequence: &Vec<i64>, prediction_strategy: fn(Vec<Vec<i64>>, &mut i64)) -> i64 {
    let derivations = get_all_derivations(sequence);

    let mut prediction = 0;
    prediction_strategy(derivations, &mut prediction);
    prediction
}

fn forwards_prediction(derivations: Vec<Vec<i64>>, prediction: &mut i64) {
    derivations.iter().rev().for_each(|level| {
        let current_last = level.iter().last().unwrap();
        *prediction += current_last;
    });
}

fn backwards_prediction(derivations: Vec<Vec<i64>>, prediction: &mut i64) {
    derivations.iter().rev().for_each(|level| {
        let current_first = level.get(0).unwrap();
        *prediction = current_first - *prediction;
    });
}

fn get_all_derivations(sequence: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut derivations = Vec::new();
    derivations.push(sequence.clone());

    for i in 0..20 {
        let current_level = derivations.get(i).unwrap();
        let next_level = derive(current_level);

        let first_elem = next_level.get(0).unwrap();
        if next_level.iter().all(|number| number == first_elem) {
            derivations.push(next_level);
            break;
        }

        derivations.push(next_level);

        if i == 20 {
            panic!("Went deeper than expected!");
        }
    }
    derivations
}

fn derive(sequence: &Vec<i64>) -> Vec<i64> {
    let mut derived_sequence = Vec::new();

    let mut seq_iterator = sequence.iter().peekable();

    while let Some(number) = seq_iterator.next() {
        match seq_iterator.peek() {
            Some(next) => derived_sequence.push(*next - number),
            None => (),
        }
    }

    derived_sequence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_all_derivations_correctly() {
        let sequence = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(get_all_derivations(&sequence).len(), 2);

        let sequence = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(get_all_derivations(&sequence).len(), 3);

        let sequence = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(get_all_derivations(&sequence).len(), 4);
    }

    #[test]
    fn derives_sequence() {
        let sequence = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(derive(&sequence), vec![3, 3, 3, 3, 3]);
    }

    #[test]
    fn predicts_value_correctly() {
        let sequence = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(predict(&sequence, forwards_prediction), 18);

        let sequence = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(predict(&sequence, forwards_prediction), 28);

        let sequence = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(predict(&sequence, forwards_prediction), 68);
    }

    #[test]
    fn predicts_previous_value_correctly() {
        let sequence = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(predict(&sequence, backwards_prediction), -3);

        let sequence = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(predict(&sequence, backwards_prediction), 5);
    }
}
