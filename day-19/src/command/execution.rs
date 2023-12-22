use super::*;

pub(super) fn execute(command: &Command, part: &Part) -> CommandResult {
    if let Some(condition) = &command.condition {
        let field = match condition.field {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
            _ => panic!(),
        };

        let is_condition_true = match condition.operation {
            Operation::LessThan => field < condition.value,
            Operation::GreaterThan => field > condition.value,
        };

        match is_condition_true {
            true => return decide_result(&command.target),
            false => return CommandResult::Next,
        };
    }
    decide_result(&command.target)
}

pub(super) fn decide_result(target: &str) -> CommandResult {
    match target {
        "R" => CommandResult::Rejected,
        "A" => CommandResult::Accepted,
        other => CommandResult::WorkflowSwitch(other.to_string()),
    }
}

#[test]
fn command_with_condition() {
    let cmd = Command {
        condition: Some(Condition {
            field: 'x',
            operation: Operation::GreaterThan,
            value: 10,
        }),
        target: "one".to_string(),
    };
    let part = Part {
        x: 20,
        m: 0,
        a: 0,
        s: 0,
        result: None,
    };
    assert_eq!(
        execute(&cmd, &part),
        CommandResult::WorkflowSwitch("one".to_string())
    );

    let part = Part {
        x: 10,
        m: 0,
        a: 0,
        s: 0,
        result: None,
    };
    assert_eq!(execute(&cmd, &part), CommandResult::Next);

    let cmd = Command {
        condition: Some(Condition {
            field: 'a',
            operation: Operation::GreaterThan,
            value: 30,
        }),
        target: "R".to_string(),
    };
    let part = Part {
        x: 10,
        m: 0,
        a: 31,
        s: 0,
        result: None,
    };
    assert_eq!(execute(&cmd, &part), CommandResult::Rejected);
}

#[test]
fn command_without_condition() {
    let part = Part {
        x: 10,
        m: 0,
        a: 31,
        s: 0,
        result: None,
    };
    let cmd = Command {
        condition: None,
        target: "R".to_string(),
    };
    assert_eq!(execute(&cmd, &part), CommandResult::Rejected);

    let cmd = Command {
        condition: None,
        target: "A".to_string(),
    };
    assert_eq!(execute(&cmd, &part), CommandResult::Accepted);

    let cmd = Command {
        condition: None,
        target: "one".to_string(),
    };
    assert_eq!(
        execute(&cmd, &part),
        CommandResult::WorkflowSwitch("one".to_string())
    );
}
