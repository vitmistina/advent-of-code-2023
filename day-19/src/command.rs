use crate::command::execution::decide_result;

use self::execution::execute;

use super::*;

mod execution;

impl Command {
    pub fn analyze(&self, class: &Class) -> Vec<(CommandResult, Class)> {
        let result = decide_result(&self.target);

        let mut output = Vec::new();
        if let Some(condition) = &self.condition {
            if result != CommandResult::Rejected {
                // create positive class
                let mut pos = (*class).clone();
                match condition.field {
                    'x' => pos.x.update_from_condition(condition, &Case::Positive),
                    'm' => pos.m.update_from_condition(condition, &Case::Positive),
                    'a' => pos.a.update_from_condition(condition, &Case::Positive),
                    's' => pos.s.update_from_condition(condition, &Case::Positive),
                    _ => panic!(),
                };
                output.push((result.clone(), pos));
            }

            // create negative class
            let mut neg = (*class).clone();
            match condition.field {
                'x' => neg.x.update_from_condition(condition, &Case::Negative),
                'm' => neg.m.update_from_condition(condition, &Case::Negative),
                'a' => neg.a.update_from_condition(condition, &Case::Negative),
                's' => neg.s.update_from_condition(condition, &Case::Negative),
                _ => panic!(),
            };
            output.push((CommandResult::Next, neg));
        } else {
            if result != CommandResult::Rejected {
                // create positive class
                let mut no_condition = (*class).clone();
                output.push((result.clone(), no_condition));
            }
        }

        output
    }
}

enum Case {
    Positive,
    Negative,
}

impl Bounds {
    fn update_from_condition(&mut self, condition: &Condition, case: &Case) {
        match (&condition.operation, case) {
            (Operation::LessThan, Case::Positive) => self.max = condition.value - 1,
            (Operation::LessThan, Case::Negative) => self.min = condition.value,
            (Operation::GreaterThan, Case::Positive) => self.min = condition.value + 1,
            (Operation::GreaterThan, Case::Negative) => self.max = condition.value,
        }
    }
}

#[test]
fn analyzes_command() {
    // new workflow, next
    let command = Command {
        condition: Some(Condition {
            field: 'a',
            operation: Operation::GreaterThan,
            value: 2000,
        }),
        target: "one".to_string(),
    };

    let class = Class::new();

    let result = command.analyze(&class);
    assert_eq!(result.len(), 2);
    assert_eq!(
        result[0],
        (
            CommandResult::WorkflowSwitch("one".to_string()),
            Class {
                x: Bounds { min: 1, max: 4000 },
                m: Bounds { min: 1, max: 4000 },
                a: Bounds {
                    min: 2001,
                    max: 4000
                },
                s: Bounds { min: 1, max: 4000 }
            }
        )
    );
    assert_eq!(
        result[1],
        (
            CommandResult::Next,
            Class {
                x: Bounds { min: 1, max: 4000 },
                m: Bounds { min: 1, max: 4000 },
                a: Bounds { min: 1, max: 2000 },
                s: Bounds { min: 1, max: 4000 }
            }
        )
    );

    // new workflow
    let command = Command {
        condition: None,
        target: "one".to_string(),
    };

    let class = Class::new();

    let result = command.analyze(&class);
    assert_eq!(result.len(), 1);
    assert_eq!(
        result[0],
        (
            CommandResult::WorkflowSwitch("one".to_string()),
            Class {
                x: Bounds { min: 1, max: 4000 },
                m: Bounds { min: 1, max: 4000 },
                a: Bounds { min: 1, max: 4000 },
                s: Bounds { min: 1, max: 4000 }
            }
        )
    );

    // next, (skip rejected)
    let command = Command {
        condition: Some(Condition {
            field: 'a',
            operation: Operation::GreaterThan,
            value: 2000,
        }),
        target: "R".to_string(),
    };

    let class = Class::new();

    let result = command.analyze(&class);
    assert_eq!(result.len(), 1);
    assert_eq!(
        result[0],
        (
            CommandResult::Next,
            Class {
                x: Bounds { min: 1, max: 4000 },
                m: Bounds { min: 1, max: 4000 },
                a: Bounds { min: 1, max: 2000 },
                s: Bounds { min: 1, max: 4000 }
            }
        )
    );

    // empty (rejected)
    let command = Command {
        condition: None,
        target: "R".to_string(),
    };

    let class = Class::new();

    let result = command.analyze(&class);
    assert_eq!(result.len(), 0);
}

impl Part {
    pub fn execute_commands(&mut self, commands: &Vec<Command>) -> CommandResult {
        let mut iter = commands.iter();
        while let Some(cmd) = iter.next() {
            match execute(cmd, &self) {
                CommandResult::Next => (),
                other => {
                    self.result = Some(other.clone());
                    return other;
                }
            }
        }
        panic!()
    }
}

#[test]
fn workflow_switch() {
    let commands = vec![
        Command {
            condition: Some(Condition {
                field: 'a',
                operation: Operation::GreaterThan,
                value: 30,
            }),
            target: "R".to_string(),
        },
        Command {
            condition: Some(Condition {
                field: 'x',
                operation: Operation::LessThan,
                value: 15,
            }),
            target: "two".to_string(),
        },
        Command {
            condition: None,
            target: "A".to_string(),
        },
    ];
    let mut part = Part {
        x: 10,
        m: 0,
        a: 30,
        s: 0,
        result: None,
    };
    assert_eq!(
        part.execute_commands(&commands),
        CommandResult::WorkflowSwitch("two".to_string())
    );

    let mut part = Part {
        x: 10,
        m: 0,
        a: 50,
        s: 0,
        result: None,
    };
    assert_eq!(part.execute_commands(&commands), CommandResult::Rejected);
    assert_eq!(part.result, Some(CommandResult::Rejected));

    let mut part = Part {
        x: 40,
        m: 0,
        a: 0,
        s: 0,
        result: None,
    };
    assert_eq!(part.execute_commands(&commands), CommandResult::Accepted);
    assert_eq!(part.result, Some(CommandResult::Accepted));
}
