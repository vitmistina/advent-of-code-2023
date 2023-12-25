use super::*;

impl UpdatesPredecessors for FlipFlop {
    fn add_predecessor(&mut self, _: String) {}
}

impl UpdatesPredecessors for ModuleBase {
    fn add_predecessor(&mut self, _: String) {}
}

impl Module for FlipFlop {
    fn get_successors(&self) -> HashSet<String> {
        self.module.outputs.clone()
    }
}

impl Module for ModuleBase {
    fn get_successors(&self) -> HashSet<String> {
        self.outputs.clone()
    }
}

impl Module for Conjunction {
    fn get_successors(&self) -> HashSet<String> {
        self.module.outputs.clone()
    }
}
