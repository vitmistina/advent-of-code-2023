use super::*;

impl Dispatcher {
    pub(super) fn process(&mut self) {
        while let Some(message) = self.queue.pop_front() {
            let node = self.graph.get_mut(&message.to).unwrap();

            let results = node.process_signal(&message);

            self.queue.extend(results);

            self.log.push(message);
        }
    }

    pub(super) fn process_with_cycles(&mut self, i: u64) {
        while let Some(message) = self.queue.pop_front() {
            if message.to == "lx".to_string() && message.signal == Signal::High {
                self.cycles.insert(message.from.clone(), i);
                // println!("{} {}", message.from, i);
            }

            let node = self.graph.get_mut(&message.to).unwrap();

            let results = node.process_signal(&message);

            self.queue.extend(results);

            self.log.push(message);
        }

        self.log.clear();
    }

    pub(super) fn count(&self) -> u64 {
        self.log.iter().filter(|m| m.signal == Signal::Low).count() as u64
            * self.log.iter().filter(|m| m.signal == Signal::High).count() as u64
    }
}
