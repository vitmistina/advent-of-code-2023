use super::*;

impl Communicate for ModuleBase {
    fn process_signal(&mut self, message: &Message) -> Vec<Message> {
        match self.t {
            Type::Button => vec![Message {
                to: "broadcaster".to_string(),
                from: "button".to_string(),
                signal: Signal::Low,
            }],
            Type::Broadcaster => self
                .outputs
                .iter()
                .map(|output| Message {
                    to: output.clone(),
                    from: self.id.clone(),
                    signal: message.signal,
                })
                .collect(),
            Type::Output => Vec::new(),
            _ => panic!(),
        }
    }
}

#[test]
fn broadcast() {
    let mut br = ModuleBase {
        id: "broadcaster".to_string(),
        t: Type::Broadcaster,
        outputs: HashSet::from(["a".to_string()]),
    };
    let result = br.process_signal(&Message {
        to: "broadcaster".to_string(),
        from: "button".to_string(),
        signal: Signal::Low,
    });
    assert_eq!(result.len(), 1);
    assert_eq!(
        result[0],
        Message {
            to: "a".to_string(),
            from: "broadcaster".to_string(),
            signal: Signal::Low
        }
    );
}

#[test]
fn button() {
    let mut br = ModuleBase {
        id: "button".to_string(),
        t: Type::Button,
        outputs: HashSet::from(["broadcaster".to_string()]),
    };
    let result = br.process_signal(&Message {
        to: "button".to_string(),
        from: "".to_string(),
        signal: Signal::Low,
    });
    assert_eq!(result.len(), 1);
    assert_eq!(
        result[0],
        Message {
            to: "broadcaster".to_string(),
            from: "button".to_string(),
            signal: Signal::Low
        }
    );
}

#[test]
fn output() {
    let mut br = ModuleBase {
        id: "output".to_string(),
        t: Type::Output,
        outputs: HashSet::new(),
    };
    let result = br.process_signal(&Message {
        to: "output".to_string(),
        from: "x".to_string(),
        signal: Signal::Low,
    });
    assert_eq!(result.len(), 0);
}
