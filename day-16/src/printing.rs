use crate::Grid;

impl Grid {
    // fn print(&self) -> Result<()> {
    //     let mut stdout = std::io::stdout();

    //     stdout.execute(MoveTo(0, 0))?;

    //     // Iterate over each cell in the grid
    //     for (y, row) in self.data.iter().enumerate() {
    //         for (x, loc) in row.iter().enumerate() {
    //             // Move the cursor to the position of the cell
    //             stdout.execute(MoveTo(x as u16, y as u16))?;

    //             // Determine the character to print based on the cell state
    //             let char_to_print = if loc.is_energized { '#' } else { '.' };

    //             // Print the character
    //             stdout.execute(Print(char_to_print))?;
    //         }
    //     }

    //     Ok(())
    // }
    // fn print(&self) -> Result<()> {
    //     let mut stdout = std::io::stdout();

    //     // Clear the screen
    //     stdout.execute(Clear(ClearType::All))?;

    //     // Iterate over each row and print it
    //     for (y, row) in self.data.iter().enumerate() {
    //         // Move the cursor to the beginning of each row
    //         stdout.execute(MoveTo(0, y as u16))?;

    //         // Create a string representation of the row
    //         let string: String = row
    //             .iter()
    //             .map(|loc| if loc.is_energized { "#" } else { "." })
    //             .collect::<String>()
    //             + "\n";

    //         // Print the row
    //         stdout.execute(Print(string))?;
    //     }

    //     Ok(())
    // }
    // fn print(&self) {
    //     self.data.iter().for_each(|row| {
    //         let string: String = row
    //             .iter()
    //             .map(|loc| if loc.is_energized { "#" } else { "." })
    //             .collect();
    //         println!("{string}");
    //     });
    // }
}
