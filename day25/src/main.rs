use std::collections::HashMap;
use std::iter::FromIterator;

use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::petgraph::graph::{NodeIndex, UnGraph};
use rustworkx_core::Result;

fn part1(input: &str) -> usize {
    let mut graph: UnGraph<(), ()> = UnGraph::new_undirected();
    let mut map: HashMap<&str, (NodeIndex, Vec<&str>)> =
        HashMap::from_iter(input.lines().map(|line| {
            let (vertex, edges) = line.split_once(": ").unwrap();
            (
                vertex,
                (graph.add_node(()), edges.split_whitespace().collect()),
            )
        }));
    for line in input.lines() {
        for e in line.split_once(": ").unwrap().1.split_whitespace() {
            if map.get(e).is_none() {
                map.insert(e, (graph.add_node(()), Vec::new()));
            }
        }
    }
    let graph_len = map.len();
    for (_, (v_i, edges)) in map.iter() {
        for e in edges.iter() {
            graph.update_edge(*v_i, map.get(e).unwrap().0, ());
        }
    }
    let min_cut_res: Result<Option<(usize, Vec<_>)>> = stoer_wagner_min_cut(&graph, |_| Ok(1));

    let partition = min_cut_res.unwrap().unwrap().1;
    partition.len() * (graph_len - partition.len())
}

fn main() {
    let binding =
        std::fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    let contents = binding.trim_start_matches("\n").trim_end_matches("\n");
    println!("result part1: {}", part1(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"
            ),
            54
        );
    }
}
