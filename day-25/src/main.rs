use std::fs::File;
use std::io::{self, Write};

use graph::Edge;

mod graph;

fn main() {
    println!("Hello, world!");

    let input = std::fs::read_to_string("input.txt").unwrap();
    // let input = "jqt: rhn xhk nvd
    //     rsh: frs pzl lsr
    //     xhk: hfx
    //     cmg: qnr nvd lhk bvb
    //     rhn: xhk bvb hfx
    //     bvb: xhk hfx
    //     pzl: lsr hfx nvd
    //     qnr: nvd
    //     ntq: jqt hfx bvb xhk
    //     nvd: lhk
    //     lsr: lhk
    //     rzs: qnr cmg lsr rsh
    //     frs: qnr lhk lsr";
    let graph = graph::Graph::from_input(&input);

    // let sorted = graph.sort_edge_scores();
    // let mut edge_scores = File::create("edge_scores.txt").expect("Unable to create file");
    // for edge in sorted {
    //     writeln!(edge_scores, "Score: {} {:#?}", edge.1, edge.0).expect("Unable to write to file");
    // }

    // Right answer for first part with my data: 603368
    let mut file = File::create("min_cuts.txt").expect("Unable to create file");
    for iteration in 0..1 {
        let mut g = graph.clone();
        println!("Iteration {}", iteration);
        if let Some(result) = g.find_cut_product() {
            writeln!(file, "{}", result).expect("Unable to write to file");
        } else {
            writeln!(file, "{:#?}", g).expect("Unable to write to file");
        }
    }
}
