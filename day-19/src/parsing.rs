use self::{part::parse_parts, workflows::parse_workflows};

use super::*;

mod part;
mod workflows;

impl System {
    pub fn parse(input: &str) -> Self {
        let mut sections = input.split("\n\n");
        Self {
            workflows: parse_workflows(sections.next().unwrap()),
            parts: parse_parts(sections.next().unwrap()),
        }
    }
}

#[test]
fn parses_full_input() {
    let input = "hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}";

    let system = System::parse(input);
    assert_eq!(system.workflows.len(), 1);
    assert_eq!(system.parts.len(), 1);
}
