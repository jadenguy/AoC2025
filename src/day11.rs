pub mod linked_list;
pub mod node_value;
pub mod reactor;

use crate::day11::{linked_list::*, node_value::*, reactor::*};
use std::collections::HashSet;
pub type Reacter = reactor::Reactor;
fn contains_noncontiguous_sequence(
    path: LinkedListNode,
    nodes_needed: &HashSet<&NodeValue>,
) -> Option<LinkedListNode> {
    let path_values: HashSet<_> = path.iter_values().collect();
    // println!(" checking {}", path);
    if nodes_needed.is_subset(&path_values) {
        println!("**saving final** {}", path);
        Some(path)
    } else {
        None
    }
}

pub fn filter_contains_noncontiguous_sequence(
    paths: Vec<LinkedListNode>,
    nodes_needed: Vec<NodeValue>,
) -> Vec<LinkedListNode> {
    let nodes_set: HashSet<_> = nodes_needed.iter().collect();
    paths
        .into_iter()
        .filter_map(|path| contains_noncontiguous_sequence(path, &nodes_set))
        .collect()
}
pub fn count_paths_containing_nodes(
    start_node: &str,
    end_node: &str,
    reactor: Reactor,
    nodes_needed: Vec<&str>,
) -> usize {
    enumerate_paths_containing_nodes(start_node, end_node, reactor, nodes_needed)
}

fn enumerate_paths_containing_nodes(
    start_node: &str,
    end_node: &str,
    reactor: Reactor,
    nodes_needed: Vec<&str>,
) -> usize {
    let paths = reactor.iter_paths(start_node, end_node);
    let valid_paths = filter_contains_noncontiguous_sequence(
        paths,
        nodes_needed.iter().map(|n| n.to_string()).collect(),
    );
    let matching_path_count = valid_paths.len();
    matching_path_count
}
#[cfg(test)]
pub mod tests {
    use crate::day11::reactor::Reactor;

    use super::*;

    const SVR: &str = "svr";
    const OUT: &str = "out";
    const DAC: &str = "dac";
    const FFT: &str = "fft";
    #[test]
    fn test_count_paths_p2_brute_force() {
        let reactor: Reactor = Reactor::from_str(sample_data_p2());
        let matching_path_count =
            enumerate_paths_containing_nodes(SVR, OUT, reactor, vec![DAC, FFT]);
        assert_eq!(matching_path_count, 2);
    }
    #[test]
    fn test_count_paths_p2() {
        let reactor: Reactor = Reactor::from_str(sample_data_p2());
        let matching_path_count = count_paths_containing_nodes(SVR, OUT, reactor, vec![DAC, FFT]);
        assert_eq!(matching_path_count, 2);
    }

    pub fn sample_data() -> Vec<&'static str> {
        r#" aaa: you hhh
            you: bbb ccc
            bbb: ddd eee
            ccc: ddd eee fff
            ddd: ggg
            eee: out
            fff: out
            ggg: out
            hhh: ccc fff iii
            iii: out"#
            .split("\n")
            .map(|x| x.trim())
            .collect()
    }
    pub fn sample_data_p2() -> Vec<&'static str> {
        r#" svr: aaa bbb
            aaa: fft
            fft: ccc
            bbb: tty
            tty: ccc
            ccc: ddd eee
            ddd: hub
            hub: fff
            eee: dac
            dac: fff
            fff: ggg hhh
            ggg: out
            hhh: out"#
            .split("\n")
            .map(|x| x.trim())
            .collect()
    }
}
