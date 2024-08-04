use std::collections::HashSet;

mod communities;
mod parsing;

pub(super) struct Graph {
    edges: HashSet<(String, String)>,
}
