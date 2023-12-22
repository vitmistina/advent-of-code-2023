use std::ops::Bound;

use super::*;

pub(super) fn analyze_ranges(input: &str) -> u64 {
    let system = System::parse(input);
    let mut buffer = vec![("in".to_string(), Class::new())];
    let mut finished = Vec::new();

    while let Some(class) = buffer.pop() {
        let workflow = system.workflows.get(&class.0).unwrap();
        let mut current_class = class.1.clone();

        for command in workflow {
            let results = command.analyze(&current_class);
            for res in results {
                match res.0 {
                    CommandResult::WorkflowSwitch(next) => buffer.push((next, res.1)),
                    CommandResult::Next => current_class = res.1,
                    CommandResult::Rejected => panic!(),
                    CommandResult::Accepted => finished.push(res.1),
                }
            }
        }
    }

    finished
        .iter()
        .map(|c| {
            [c.x, c.m, c.a, c.s]
                .iter()
                .map(Bounds::get_range)
                .product::<u64>()
        })
        .sum()
}

impl Bounds {
    fn get_range(&self) -> u64 {
        self.max - self.min + 1
    }
}

#[test]
fn integration() {
    let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
    assert_eq!(analyze_ranges(input), 167409079868000);
}
