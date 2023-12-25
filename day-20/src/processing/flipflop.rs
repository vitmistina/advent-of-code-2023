use super::*;

impl State {
    fn toggle(&self) -> Self {
        match self {
            State::On => State::Off,
            State::Off => State::On,
        }
    }

    fn map_flip_flop(&self) -> Signal {
        match self {
            State::On => Signal::Low,
            State::Off => Signal::High,
        }
    }
}

impl Communicate for FlipFlop {
    fn process_signal(&mut self, message: &Message) -> Vec<Message> {
        match message.signal {
            Signal::High => Vec::new(),
            Signal::Low => {
                let signal = self.state.map_flip_flop();
                self.state = self.state.toggle();
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
    }
}
#[test]
fn processes_message() {
    let mut ff = FlipFlop {
        module: ModuleBase {
            id: "ff".to_string(),
            t: Type::FlipFlop,
            outputs: HashSet::from(["a".to_string()]),
        },
        state: State::Off,
    };
    let result = ff.process_signal(&Message {
        to: "ff".to_string(),
        from: "x".to_string(),
        signal: Signal::High,
    });
    assert_eq!(result.len(), 0);
    assert_eq!(ff.state, State::Off);

    let result = ff.process_signal(&Message {
        to: "ff".to_string(),
        from: "x".to_string(),
        signal: Signal::Low,
    });
    assert_eq!(
        result[0],
        Message {
            to: "a".to_string(),
            from: "ff".to_string(),
            signal: Signal::High
        }
    );
    assert_eq!(ff.state, State::On);

    let result = ff.process_signal(&Message {
        to: "ff".to_string(),
        from: "x".to_string(),
        signal: Signal::Low,
    });
    assert_eq!(
        result[0],
        Message {
            to: "a".to_string(),
            from: "ff".to_string(),
            signal: Signal::Low
        }
    );
    assert_eq!(ff.state, State::Off);
}
