use crate::day11::node_value::*;

use super::*;
use std::collections::HashMap;
#[derive(Debug, Clone)]
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
    pub fn path_iter(&self, start_node: &str, end_node: &str) -> ReactorPathIterator {
        ReactorPathIterator {
            reactor: self.clone(),
            to_check: vec![LinkedListNode::from_str_with_child(start_node, None)],
            end_node: NodeValue::from(end_node),
        }
    }

    pub(crate) fn generate_node_table(&self) -> HashMap<NodeValue, usize> {
        let mut map = HashMap::new();
        for (k, v_list) in &self.network {
            map.insert(k.to_owned(), 0);
            for v in v_list {
                map.insert(v.to_owned(), 0);
            }
        }
        map
    }
}

#[derive(Debug)]
pub struct ReactorPathIterator {
    reactor: Reacter,
    to_check: Vec<LinkedListNode>,
    end_node: NodeValue,
}
impl Iterator for ReactorPathIterator {
    type Item = LinkedListNode;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.to_check.pop() {
            if let Some(connections) = self.reactor.network.get(&node.tail) {
                for connection in connections {
                    let new_link = node.append(connection.to_owned());
                    // println!("  {}", new_link);
                    if connection == &self.end_node {
                        return Some(new_link);
                    } else {
                        self.to_check.push(new_link)
                    }
                }
            }
        }
        None
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
        let paths = reactor.path_iter("you", "out").count();
        assert_eq!(paths, 5);
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
