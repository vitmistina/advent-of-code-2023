pub struct Inits {
    pub stacks_map: String,
    pub crane_instructions: String,
}

impl Inits {
    pub fn from(input: &str) -> Inits {
        let unified_newlines = input.replace("\r", "");
        let parts: Vec<_> = unified_newlines.split("\n\n").collect();
        Inits {
            stacks_map: String::from(parts[0]),
            crane_instructions: String::from(parts[1]),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    #[test]
    fn data_loaded() {
        let input = " 1   2   3 

        move 1 from 2 to 1";
        let output = Inits::from(input);
        assert!(output.stacks_map.len() > 0);
        assert!(output.crane_instructions.len() > 0);
    }
    #[test]
    #[ignore]
    fn full_data_load() {
        let contents: String =
            fs::read_to_string("data.txt").expect("Should have been able to read the file");

        let output = Inits::from(&contents);
        assert!(output.stacks_map.len() > 0);
        assert!(output.crane_instructions.len() > 0);
    }
}
