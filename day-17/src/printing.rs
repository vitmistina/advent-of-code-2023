use super::*;

impl Grid {
    pub fn print(&self, directions: &Vec<Direction>) {
        let mut print_layer: Vec<String> = self
            .data
            .get(&(1, Direction::Right))
            .unwrap()
            .iter()
            .map(|row| row.iter().map(|node| node.heat_loss.to_string()).collect())
            .collect();
        // print_layer
        //     .iter_mut()
        //     .for_each(|row| row.iter_mut().for_each(|node| node.current_score = Some(0)));

        let mut current_coord = Coordinate { x: 0, y: 0 };
        for dir in directions {
            let (new_x, new_y, x_len, y_len) = self.get_coords_for_direction(dir, &current_coord);
            let (x, y) = { (new_x.unwrap(), new_y.unwrap()) };
            current_coord = Coordinate { x, y };
            let arrow = match dir {
                Direction::Right => ">",
                Direction::Down => "v",
                Direction::Up => "^",
                Direction::Left => "<",
            };
            print_layer[y].replace_range(x..x + 1, arrow);
        }

        print_layer.iter().for_each(|line| {
            // let line: String = row
            //     .chars()
            //     .map(|char| node.current_score.unwrap().to_string())
            //     .collect();
            println!("{line}");
        });
    }
}
