mod warehouse;
use std::fs;

use warehouse::Warehouse;
fn main() {
    let contents: String = fs::read_to_string("data.txt").expect("Should have been able to read the file");
    let warehouse = Warehouse::from(&contents).start_crane();
    println!("Top crates are: {}", warehouse.read_top());
}
