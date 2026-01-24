pub mod linked_list;
use std::collections::HashMap;

use crate::day11::linked_list::*;

#[derive(Debug)]
pub struct Reactor {
    pub network: HashMap<NodeValue, Vec<NodeValue>>,
}
impl Reactor {
    pub fn from_str(lines: Vec<&str>) -> Reactor {
        let mut network = HashMap::new();
        for line in lines {
            let (key, value) = line.split_once(": ").unwrap();
            network.insert(
                NodeValue::from(key),
                value.split(" ").map(|x| NodeValue::from(x)).collect(),
            );
        }
        Reactor { network }
    }
    pub fn from_string(lines: Vec<String>) -> Reactor {
        Reactor::from_str(lines.iter().map(|x| x.as_str()).collect())
    }
    pub fn count_paths(&self, start_node: &str, end_node: &str) -> usize {
        self.iter_paths(start_node, end_node).len()
    }

    fn iter_paths(&self, start_node: &str, end_node: &str) -> Vec<LinkedListNode> {
        let mut to_check: Vec<LinkedListNode> =
            vec![LinkedListNode::from_str_with_child(start_node, None)];
        let mut successful: Vec<LinkedListNode> = Vec::new();
        while let Some(node) = to_check.pop() {
            if let Some(connections) = self.network.get(&node.tail()) {
                for connection in connections {
                    let new_link = node.append(connection.to_owned());
                    println!("  {}", new_link);
                    if connection == end_node {
                        println!("saving {}", new_link);
                        successful.push(new_link);
                    } else {
                        to_check.push(new_link)
                    }
                }
            }
        }
        successful
    }
    // fn count_paths_brute_force(&self, start_node: &str, end_node: &str) -> u64 {
    //     let mut visited: Vec<String> = Vec::new();
    //     let mut to_check: Vec<String> = vec![start_node.to_string()];
    //     let mut successful: u64 = 0;
    //     while let Some(node) = to_check.pop() {
    //         if let Some(connections) = self.network.get(&node) {
    //             for connection in connections {
    //                 to_check.push(connection.clone());
    //             }
    //             // println!("{:?}", to_check)
    //         }
    //         if node == end_node {
    //             successful += 1;
    //         }
    //         visited.push(node);
    //     }
    //     successful
    // }
}
fn contains_noncontiguous_sequence(
    path: LinkedListNode,
    nodes_needed: &[String],
) -> Option<LinkedListNode> {
    println!("{}", path);
    let mut last_node_position = 0;
    for node in nodes_needed {
        if let Some(node_position) = path
            .iter_values()
            .map(|d| d.to_owned())
            .position(|n| n == *node)
            && node_position >= last_node_position
        {
            last_node_position = node_position;
        } else {
            return None;
        }
    }
    Some(path)
}

pub fn filter_contains_noncontiguous_sequence(
    paths: Vec<LinkedListNode>,
    nodes_needed: Vec<NodeValue>,
) -> Vec<LinkedListNode> {
    paths
        .into_iter()
        .filter_map(|path| contains_noncontiguous_sequence(path, &nodes_needed))
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    // stolen from somewhere?
    macro_rules! map {
        ($($k:expr => $v:expr),* $(,)?) => {
            HashMap::from([
                $((NodeValue::from($k), $v.iter().map(|&s| NodeValue::from(s)).collect()),)*
            ])
        };
    }
    #[test]
    fn test_count_paths() {
        let reactor: Reactor = Reactor::from_str(sample_data());
        let paths = reactor.count_paths("you", "out");
        assert_eq!(paths, 5);
    }
    #[test]
    fn test_count_paths_p2() {
        let reactor: Reactor = Reactor::from_str(sample_data_p2());
        println!("{:?}", reactor.network);
        let paths = reactor.iter_paths("svr", "out");
        let nodes_needed = vec!["dac".to_string(), "fft".to_string()];

        let valid_paths = filter_contains_noncontiguous_sequence(paths, nodes_needed);
        let matching_path_count = valid_paths.len();
        assert_eq!(matching_path_count, 2);
    }
    #[test]
    fn test_parse_reactor() {
        let sample_data = sample_data();
        let reactor: Reactor = Reactor::from_str(sample_data);
        let n = map!(
            "aaa"=> ["you", "hhh"],
            "you"=> ["bbb", "ccc"],
            "bbb"=> ["ddd", "eee"],
            "ccc"=> ["ddd", "eee", "fff"],
            "ddd"=> ["ggg"],
            "eee"=> ["out"],
            "fff"=> ["out"],
            "ggg"=> ["out"],
            "hhh"=> ["ccc", "fff", "iii"],
            "iii"=> ["out"],
        );
        assert_eq!(reactor.network, n);
    }
    fn sample_data() -> Vec<&'static str> {
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
    fn sample_data_p2() -> Vec<&'static str> {
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
