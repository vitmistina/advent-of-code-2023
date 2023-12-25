use self::{
    conjunction::prepare_conjuctions,
    module::{add_outputs, parse_module},
};

use super::*;

mod conjunction;
mod module;

pub fn parse_inputs(input: &str) -> HashMap<String, Box<dyn Module>> {
    let mut modules = input
        .lines()
        .map(|line| parse_module(&line).expect("Module needs to be OK"))
        .collect();
    add_outputs(&mut modules);
    prepare_conjuctions(&mut modules);
    modules
}

#[test]
fn parses_lines() {
    let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    let result = parse_inputs(input);

    assert_eq!(result.len(), 6);
}
