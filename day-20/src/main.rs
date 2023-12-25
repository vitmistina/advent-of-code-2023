use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

use lcm::lowest_common_multiple;
use parsing::parse_inputs;

mod common;
mod dispatcher;
mod lcm;
mod parsing;
mod processing;

fn main() {
    let input = &fs::read_to_string("input.txt").expect("The file should be there");
    let result = integrate(input);
    println!("Hello, world! {result}");

    let result = integrate_for_rx(input);
    println!("Hello, world! {result}");
}

enum Type {
    Button,
    Broadcaster,
    FlipFlop,
    Conjunction,
    Output,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Signal {
    Low,
    High,
}

#[derive(Debug, PartialEq)]
enum State {
    On,
    Off,
}

trait Communicate {
    fn process_signal(&mut self, message: &Message) -> Vec<Message>;
}

trait UpdatesPredecessors {
    fn add_predecessor(&mut self, id: String);
}

trait Module: Communicate + UpdatesPredecessors {
    fn get_successors(&self) -> HashSet<String>;
}

struct ModuleBase {
    id: String,
    t: Type,
    outputs: HashSet<String>,
}
struct FlipFlop {
    module: ModuleBase,
    state: State,
}
struct Conjunction {
    module: ModuleBase,
    recent_inputs: HashMap<String, Signal>,
}

#[derive(Debug, PartialEq)]
struct Message {
    from: String,
    to: String,
    signal: Signal,
}

struct Dispatcher {
    log: Vec<Message>,
    queue: VecDeque<Message>,
    graph: HashMap<String, Box<dyn Module>>,
    cycles: HashMap<String, u64>,
}

fn integrate(input: &str) -> u64 {
    let mut dispatcher = Dispatcher {
        log: Vec::new(),
        queue: VecDeque::new(),
        graph: parse_inputs(input),
        cycles: HashMap::new(),
    };

    for _ in 0..1000 {
        dispatcher.queue.extend([Message {
            to: "broadcaster".to_string(),
            from: "button".to_string(),
            signal: Signal::Low,
        }]);
        dispatcher.process();
    }
    dispatcher.count()
}

fn integrate_for_rx(input: &str) -> u64 {
    let mut dispatcher = Dispatcher {
        log: Vec::new(),
        queue: VecDeque::new(),
        graph: parse_inputs(input),
        cycles: HashMap::new(),
    };

    for i in 1..100000 {
        dispatcher.queue.extend([Message {
            to: "broadcaster".to_string(),
            from: "button".to_string(),
            signal: Signal::Low,
        }]);
        dispatcher.process_with_cycles(i);
        if dispatcher.cycles.len() == 4 {
            break;
        }
    }
    dispatcher
        .cycles
        .iter()
        .fold(1, |acc, current| lowest_common_multiple(acc, *current.1))
}

#[test]
fn processes_queue() {
    let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    let mut dispatcher = Dispatcher {
        log: Vec::new(),
        queue: VecDeque::from([Message {
            to: "broadcaster".to_string(),
            from: "button".to_string(),
            signal: Signal::Low,
        }]),
        graph: parse_inputs(input),
        cycles: HashMap::new(),
    };

    dispatcher.process();

    assert_eq!(dispatcher.log.len(), 12)
}

#[test]
fn integrate_first() {
    let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    assert_eq!(integrate(input), 32000000)
}

#[test]
fn integrate_second() {
    let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    assert_eq!(integrate(input), 11687500)
}
