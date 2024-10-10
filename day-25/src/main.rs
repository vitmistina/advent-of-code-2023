mod graph;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let graph = graph::Graph::from_input(&input);

    // Right answer for first part with my data: 603368
    let mut g = graph.clone();
    if let Some(result) = g.find_cut_product() {
        println!("{}", result);
    } else {
        println!("Unable to finish {:#?}", g);
    }
}
