use crate::day11::node_value::*;
use std::fmt;
#[derive(Debug)]
pub struct LinkedListNode {
    pub value: NodeValue,
    pub child: Option<Box<LinkedListNode>>,
    pub tail: NodeValue,
}
impl LinkedListNode {
    pub fn from_str_with_child(value: &str, child: Option<LinkedListNode>) -> LinkedListNode {
        let value = NodeValue::from(value);
        if let Some(c) = child {
            let tail = c.tail.to_owned();
            LinkedListNode {
                child: Some(Box::from(c)),
                tail,
                value,
            }
        } else {
            LinkedListNode {
                tail: value.to_owned(),
                child: None,
                value,
            }
        }
    }
    pub fn new(value: NodeValue) -> LinkedListNode {
        LinkedListNode {
            tail: value.to_owned(),
            child: None,
            value,
        }
    }
    pub fn from_vec(vec: Vec<NodeValue>) -> Option<LinkedListNode> {
        let mut predecessor: Option<LinkedListNode> = None;
        let tail = vec.last()?;
        for value in vec.iter().rev().map(|v| v.to_owned()) {
            let mut child = None;
            if let Some(parent_node) = predecessor {
                child = Some(Box::new(parent_node));
            }
            let new = LinkedListNode {
                child,
                value,
                tail: tail.to_owned(),
            };
            predecessor = Some(new);
        }
        predecessor
    }
}

impl fmt::Display for LinkedListNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)?;

        if let Some(next) = self.child.as_deref() {
            write!(f, "=>{}", next)?;
        }

        Ok(())
    }
}

impl LinkedListNode {
    pub fn append(&self, value: NodeValue) -> LinkedListNode {
        let mut vec = Vec::from_iter(self.iter_values().map(|v| NodeValue::from(v)));
        vec.push(value);
        LinkedListNode::from_vec(vec).unwrap()
    }
    pub fn prepend(&self, value: NodeValue) -> LinkedListNode {
        let mut vec = vec![value];
        vec.extend(self.iter_values().map(|v| NodeValue::from(v)));
        LinkedListNode::from_vec(vec).unwrap()
    }
    pub fn clone(&self) -> LinkedListNode {
        LinkedListNode::from_vec(Vec::from_iter(
            self.iter_values().map(|v| NodeValue::from(v)),
        ))
        .unwrap()
    }
    pub fn iter(&self) -> LinkedListNodeIterator<'_> {
        LinkedListNodeIterator { cursor: Some(self) }
    }
    pub fn iter_values(&self) -> LinkedListNodeValueIterator<'_> {
        LinkedListNodeValueIterator { cursor: Some(self) }
    }
}
#[derive(Debug)]
pub struct LinkedListNodeIterator<'a> {
    cursor: Option<&'a LinkedListNode>,
}
impl<'a> Iterator for LinkedListNodeIterator<'a> {
    type Item = &'a LinkedListNode;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.cursor?;
        self.cursor = current.child.as_deref();
        Some(current)
    }
}

#[derive(Debug)]
pub struct LinkedListNodeValueIterator<'a> {
    cursor: Option<&'a LinkedListNode>,
}
impl<'a> Iterator for LinkedListNodeValueIterator<'a> {
    type Item = &'a NodeValue;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.cursor?;
        self.cursor = current.child.as_deref();
        Some(&current.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_linked_list_node_from_vec() {
        let root = LinkedListNode::from_vec(vec![
            NodeValue::from("root"),
            NodeValue::from("child"),
            NodeValue::from("leaf"),
        ])
        .unwrap();
        let value_list = Vec::from_iter(root.iter_values());
        assert_eq!(value_list, vec!["root", "child", "leaf"])
    }
    #[test]
    fn test_linked_list_node_clone() {
        let root = LinkedListNode::from_vec(vec![
            NodeValue::from("root"),
            NodeValue::from("child"),
            NodeValue::from("leaf"),
        ])
        .unwrap();
        let clone = root.clone();
        let root_val_list = Vec::from_iter(root.iter_values());
        let clone_val_list = Vec::from_iter(clone.iter_values());
        assert_eq!(clone_val_list, root_val_list)
    }
    #[test]
    fn test_linked_list_node() {
        let leaf = LinkedListNode {
            value: NodeValue::from("leaf"),
            tail: NodeValue::from("leaf"),
            child: None,
        };
        let child = LinkedListNode {
            value: NodeValue::from("child"),
            tail: NodeValue::from("child"),
            child: Some(Box::from(leaf)),
        };
        let root = LinkedListNode {
            value: NodeValue::from("root"),
            tail: NodeValue::from("root"),
            child: Some(Box::from(child)),
        };
        let l_list = Vec::from_iter(root.iter_values());
        assert_eq!(l_list, vec!["root", "child", "leaf"])
    }
}
