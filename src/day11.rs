pub mod linked_list;
pub mod node_value;
pub mod reactor;

use crate::day11::{linked_list::*, node_value::*, reactor::*};
use std::{
    collections::{HashMap, HashSet},
    vec,
};
pub type Reacter = reactor::Reactor;
pub fn filter_superset_paths(
    paths: ReactorPathIterator,
    nodes_needed: Vec<NodeValue>,
) -> Vec<LinkedListNode> {
    let nodes_set: HashSet<_> = nodes_needed.iter().collect();
    paths
        .into_iter()
        .filter(|p| {
            let node_set: &HashSet<&String> = &nodes_set;
            node_set.is_subset(&p.iter_values().collect())
        })
        .collect()
}
pub fn count_paths_containing_nodes(
    start_node: &str,
    end_node: &str,
    reactor: Reactor,
    nodes_needed: Vec<&str>,
) -> usize {
    // enumerate_paths_containing_nodes(start_node, end_node, reactor, nodes_needed)
    // sum_iter_paths(start_node, end_node, reactor, nodes_needed)
    count_paths_passing_through_x(start_node, end_node, reactor, nodes_needed)
}
pub fn sum_iter_paths(
    start_node: &str,
    end_node: &str,
    reactor: Reactor,
    nodes_needed: Vec<&str>,
) -> usize {
    let node_set: HashSet<_> = nodes_needed.iter().map(|&n| NodeValue::from(n)).collect();
    fn path_contains_all_nodes(path: &LinkedListNode, node_set: &HashSet<String>) -> bool {
        // println!("{}", path);
        let other: HashSet<NodeValue> = path.iter_values().cloned().collect();
        node_set.is_subset(&other)
    }
    let a_path_containing_ordered_nodes_assuming_no_list = reactor
        .path_iter(start_node, end_node)
        .find(|p| path_contains_all_nodes(p, &node_set));
    if let Some(sample_path) = a_path_containing_ordered_nodes_assuming_no_list {
        let mut sequence = vec![start_node];
        sequence.extend(
            sample_path
                .iter_values()
                .filter(|value| node_set.contains(value.as_str()))
                .map(|v| v.as_str()),
        );
        sequence.push(end_node);
        // println!("{:?}", sequence);
        sequence.reverse();
        let mut result = 1;
        let mut start = sequence.pop().unwrap();
        while let Some(end) = sequence.pop() {
            let path_iter: Vec<LinkedListNode> = reactor.path_iter(start, end).collect();
            // for path in &path_iter {
            // println!("{}", path);
            // }

            result *= path_iter.len();
            start = end;
        }
        return result;
    }
    0
}
pub fn enumerate_paths_containing_nodes(
    start_node: &str,
    end_node: &str,
    reactor: Reactor,
    nodes_needed: Vec<&str>,
) -> usize {
    let paths = reactor.path_iter(start_node, end_node);
    let valid_paths =
        filter_superset_paths(paths, nodes_needed.iter().map(|n| n.to_string()).collect());
    let matching_path_count = valid_paths.len();
    matching_path_count
}

pub fn count_paths_passing_through_x(
    start_node: &str,
    end_node: &str,
    reactor: Reactor,
    nodes: Vec<&str>,
) -> usize {
    let mut node_set: HashSet<&str> = HashSet::from_iter(nodes);
    node_set.insert(start_node);
    node_set.insert(end_node);
    let clean_map = reactor.generate_node_table();
    let strata = calculate_node_strata(start_node, &reactor, &clean_map);
    {
        let mut strata_vec: Vec<(NodeValue, usize)> = strata
            .iter()
            .filter_map(|(k, v)| Some((k.to_owned(), v.to_owned())))
            .collect();
        strata_vec.sort_by_key(|(_, v)| v.to_owned());
        // println!("{:?}", strata_vec);
    }
    let mut strata_vec: Vec<(NodeValue, usize)> = strata
        .iter()
        .filter_map(|(k, v)| {
            let find = node_set.contains(k.as_str());
            match find {
                true => Some((k.to_owned(), v.to_owned())),
                _ => None,
            }
        })
        .collect();
    strata_vec.sort_by_key(|(_, v)| v.to_owned());
    // println!("{:?}", strata_vec);
    let mut check_pairs = Vec::new();
    let first = strata_vec.iter().take(strata_vec.len() - 1);
    let second = strata_vec.iter().skip(1);
    let pair = first.zip(second);
    for (a, b) in pair {
        check_pairs.push(((a.0.to_owned()), (b.0.to_owned())));
    }
    let mut seed = 1;
    let mut output = 1;
    for (a, b) in check_pairs {
        output = calculate_max_distance_to_node(seed, &a, &b, &reactor, &clean_map)[&b];
        seed = output;
    }
    output
}

fn calculate_max_distance_to_node(
    seed: usize,
    start_node: &NodeValue,
    end_node: &NodeValue,
    reactor: &Reactor,
    clean_map: &HashMap<NodeValue, usize>,
) -> HashMap<NodeValue, usize> {
    let mut map = clean_map.clone();
    let k = &NodeValue::from(start_node);
    *map.get_mut(k).unwrap() += seed;
    let mut to_check: Vec<NodeValue> = vec![NodeValue::from(start_node)];
    while to_check.len() > 0 {
        let k = &to_check.remove(0).to_owned();
        if let Some(children) = reactor.network.get(k) {
            let k_value = *map.get(k).unwrap();
            if k_value > 0 {
                for child in children {
                    // println!("added {} from {} to {}", k_value, k, child);
                    *map.get_mut(child).unwrap() += k_value;
                    if !to_check.contains(child) {
                        to_check.push(child.to_owned());
                    }
                }
            }
            if k != end_node {
                *map.get_mut(k).unwrap() = 0;
            }
        }
    }
    map
}
fn calculate_node_strata(
    start_node: &str,
    reactor: &Reactor,
    clean_map: &HashMap<NodeValue, usize>,
) -> HashMap<NodeValue, usize> {
    let mut map = clean_map.clone();
    let k = &NodeValue::from(start_node);
    *map.get_mut(k).unwrap() += 1;
    let mut to_check: Vec<NodeValue> = vec![NodeValue::from(start_node)];
    while to_check.len() > 0 {
        let k = &to_check.remove(0);
        if let Some(children) = reactor.network.get(k) {
            let k_value = *map.get(k).unwrap();
            if k_value > 0 {
                for child in children {
                    if map[child] <= k_value {
                        *map.get_mut(child).unwrap() = k_value + 1;
                    }
                    if !to_check.contains(child) {
                        to_check.push(child.to_owned());
                    }
                }
            }
        }
    }
    map
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
    fn test_count_paths_mathy() {
        let reactor: Reactor = Reactor::from_str(sample_data_p2());
        let matching_path_count = count_paths_passing_through_x(SVR, OUT, reactor, vec![DAC, FFT]);
        assert_eq!(matching_path_count, 2);
    }
    #[test]
    fn test_count_paths_enumerated_paths() {
        let reactor: Reactor = Reactor::from_str(sample_data_p2());
        let matching_path_count =
            enumerate_paths_containing_nodes(SVR, OUT, reactor, vec![DAC, FFT]);
        assert_eq!(matching_path_count, 2);
    }
    #[test]
    fn test_sum_iter_paths() {
        let reactor: Reactor = Reactor::from_str(sample_data_p2());
        let matching_path_count = sum_iter_paths(SVR, OUT, reactor, vec![DAC, FFT]);
        assert_eq!(matching_path_count, 2);
    }
    #[test]
    fn test_count_paths() {
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
