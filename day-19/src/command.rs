use self::execution::execute;

use super::*;

mod execution;

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
