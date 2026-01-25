use crate::day11::node_value::*;

use super::*;
use std::collections::HashMap;
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

    pub fn iter_paths(&self, start_node: &str, end_node: &str) -> Vec<LinkedListNode> {
        let mut to_check: Vec<LinkedListNode> =
            vec![LinkedListNode::from_str_with_child(start_node, None)];
        let mut successful: Vec<LinkedListNode> = Vec::new();
        while let Some(node) = to_check.pop() {
            if let Some(connections) = self.network.get(&node.tail()) {
                for connection in connections {
                    let new_link = node.append(connection.to_owned());
                    // println!("  {}", new_link);
                    if connection == end_node {
                        if successful.len() % 100 == 0 {
                            println!("saving {}", new_link);
                        };
                        successful.push(new_link);
                    } else {
                        to_check.push(new_link)
                    }
                }
            }
        }
        successful
    }
}

#[cfg(test)]
mod tests {
    use crate::day11::tests as super_tests;

    use super::*;
    use std::collections::HashMap;
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
        let reactor: Reactor = Reactor::from_str(super_tests::sample_data());
        let paths = reactor.iter_paths("you", "out");
        assert_eq!(paths.len(), 5);
    }
    #[test]
    fn test_parse_reactor() {
        let sample_data = super_tests::sample_data();
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
}
