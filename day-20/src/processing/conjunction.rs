use super::*;

impl Communicate for Conjunction {
    fn process_signal(&mut self, message: &Message) -> Vec<Message> {
        self.recent_inputs
            .insert(message.from.clone(), message.signal);
        let is_every_high = self.recent_inputs.values().all(|sig| sig == &Signal::High);
        let signal = if is_every_high {
            Signal::Low
        } else {
            Signal::High
        };
        self.module
            .outputs
            .iter()
            .map(|output| Message {
                to: output.clone(),
                from: self.module.id.clone(),
                signal,
            })
            .collect()
    }
}

#[test]
fn processes_message() {
    let id = "cj".to_string();
    let mut cj = Conjunction {
        module: ModuleBase {
            id: id.clone(),
            t: Type::Conjunction,
            outputs: HashSet::from(["a".to_string()]),
        },
        recent_inputs: HashMap::from([
            ("x".to_string(), Signal::Low),
            ("y".to_string(), Signal::Low),
        ]),
    };
    let result = cj.process_signal(&Message {
        to: id.clone(),
        from: "x".to_string(),
        signal: Signal::High,
    });
    assert_eq!(result.len(), 1);
    assert_eq!(
        result[0],
        Message {
            to: "a".to_string(),
            from: id.clone(),
            signal: Signal::High
        }
    );
    assert_eq!(cj.recent_inputs.get("x").unwrap(), &Signal::High);

    let result = cj.process_signal(&Message {
        to: id.clone(),
        from: "y".to_string(),
        signal: Signal::High,
    });
    assert_eq!(result.len(), 1);
    assert_eq!(
        result[0],
        Message {
            to: "a".to_string(),
            from: id.clone(),
            signal: Signal::Low
        }
    );
    assert_eq!(cj.recent_inputs.get("y").unwrap(), &Signal::High);

    let result = cj.process_signal(&Message {
        to: id.clone(),
        from: "x".to_string(),
        signal: Signal::Low,
    });
    assert_eq!(result.len(), 1);
    assert_eq!(
        result[0],
        Message {
            to: "a".to_string(),
            from: id.clone(),
            signal: Signal::High
        }
    );
    assert_eq!(cj.recent_inputs.get("x").unwrap(), &Signal::Low);
}
