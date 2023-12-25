use super::*;

pub(super) fn parse_module(input: &str) -> Result<(String, Box<dyn Module>), String> {
    let parts: Vec<&str> = input.split("->").collect();
    if parts.len() != 2 {
        return Err("Invalid format".to_string());
    }

    let module_type = parts[0].trim().chars().nth(0).unwrap();
    let is_broadcaster = parts[0].trim() == "broadcaster";
    let id = if is_broadcaster {
        parts[0].trim().to_string()
    } else {
        parts[0].trim()[1..].to_string()
    };
    let outputs: HashSet<String> = parts[1]
        .trim()
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    let boxed: Box<dyn Module> = match module_type {
        '%' => Box::new(FlipFlop {
            module: ModuleBase {
                id: id.clone(),
                t: Type::FlipFlop,
                outputs,
            },
            state: State::Off,
        }),
        '&' => Box::new(Conjunction {
            module: ModuleBase {
                id: id.clone(),
                t: Type::Conjunction,
                outputs,
            },
            recent_inputs: HashMap::new(),
        }),
        _ => Box::new(ModuleBase {
            id: id.clone(),
            t: if is_broadcaster {
                Type::Broadcaster
            } else {
                Type::Button
            },
            outputs,
        }),
    };

    Ok((id, boxed))
}

pub(super) fn add_outputs(graph: &mut HashMap<String, Box<dyn Module>>) {
    let modules = graph.keys().map(|k| k.clone()).collect::<Vec<_>>();

    for module in modules {
        let outputs = graph.get(&module).unwrap().get_successors();
        for successor in outputs {
            match graph.contains_key(&successor) {
                true => {}
                false => {
                    graph.insert(
                        successor.clone(),
                        Box::new(ModuleBase {
                            id: successor.clone(),
                            t: Type::Output,
                            outputs: HashSet::new(),
                        }),
                    );
                }
            }
        }
    }
}

#[test]
fn parse_line() {
    let input = "broadcaster -> hd, gs, fc, sx";
    let module = parse_module(input);
    assert!(module.is_ok());
    let input = "&qb -> bf, kf, hd, nl, pm, lb";
    let module = parse_module(input);
    assert!(module.is_ok());
    let input = "%hd -> ms, qb";
    let module = parse_module(input);
    assert!(module.is_ok());
}
