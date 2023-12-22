use super::*;

impl Part {
    fn parse(input: &str) -> Self {
        //.replace("{", "").replace("}", "")
        let mut params = input[1..input.len() - 1]
            .split(",")
            .map(|param| &param[2..]);
        Self {
            x: params.next().unwrap().parse().unwrap(),
            m: params.next().unwrap().parse().unwrap(),
            a: params.next().unwrap().parse().unwrap(),
            s: params.next().unwrap().parse().unwrap(),
            result: None,
        }
    }
}

pub(super) fn parse_parts(input: &str) -> Vec<Part> {
    input.lines().map(|line| Part::parse(line)).collect()
}

#[test]
fn parses_part() {
    let input = "{x=787,m=2655,a=1222,s=2876}";
    assert_eq!(
        Part::parse(input),
        Part {
            x: 787,
            m: 2655,
            a: 1222,
            s: 2876,
            result: None
        }
    );
}
