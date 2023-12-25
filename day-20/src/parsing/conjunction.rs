use super::*;

impl UpdatesPredecessors for Conjunction {
    fn add_predecessor(&mut self, id: String) {
        self.recent_inputs.insert(id.clone(), Signal::Low);
    }
}

pub(super) fn prepare_conjuctions(graph: &mut HashMap<String, Box<dyn Module>>) {
    let modules = graph.keys().map(|k| k.clone()).collect::<Vec<_>>();

    for module in modules {
        let outputs = graph.get(&module).unwrap().get_successors();
        for successor in outputs {
            graph
                .get_mut(&successor)
                .expect("Didn't find module")
                .add_predecessor(module.clone());
        }
    }
}

#[test]
fn conjunction_has_inputs_prepared() {
    //TODO: Mocks with Automock

    let cj = Conjunction {
        module: ModuleBase {
            id: "cj".to_string(),
            t: Type::Conjunction,
            outputs: HashSet::new(),
        },
        recent_inputs: HashMap::new(),
    };

    let br = ModuleBase {
        id: "broadcaster".to_string(),
        t: Type::Broadcaster,
        outputs: HashSet::from(["cj".to_string(), "ff".to_string()]),
    };

    let ff = FlipFlop {
        module: ModuleBase {
            id: "ff".to_string(),
            t: Type::FlipFlop,
            outputs: HashSet::from(["cj".to_string()]),
        },
        state: State::Off,
    };

    let b_id = cj.module.id.clone();
    let b: Box<dyn Module> = Box::new(cj);

    let mut graph: HashMap<String, Box<dyn Module>> = HashMap::from([
        (b_id, b),
        (br.id.clone(), Box::new(br)),
        (ff.module.id.clone(), Box::new(ff)),
    ]);

    prepare_conjuctions(&mut graph);
}
