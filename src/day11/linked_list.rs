use core::fmt;
pub type NodeValue = String;

#[derive(Debug)]
pub struct LinkedListNode {
    pub value: NodeValue,
    pub child: Option<Box<LinkedListNode>>,
}
impl LinkedListNode {
    pub fn from_str_with_child(value: &str, child: Option<LinkedListNode>) -> LinkedListNode {
        LinkedListNode {
            value: value.to_string(),
            child: match child {
                Some(c) => Some(Box::from(c)),
                None => None,
            },
        }
    }
    pub fn new(value: NodeValue) -> LinkedListNode {
        LinkedListNode { value, child: None }
    }
    pub fn from_vec(vec: Vec<NodeValue>) -> Option<LinkedListNode> {
        let mut current: Option<LinkedListNode> = None;
        for value in vec.iter().rev().map(|v| v.to_owned()) {
            let mut child = None;
            if let Some(parent_node) = current {
                child = Some(Box::new(parent_node));
            }
            let new = LinkedListNode { child, value };
            current = Some(new);
        }
        current
    }

    pub fn tail(&self) -> NodeValue {
        match &self.child {
            Some(c) => c.tail(),
            None => self.value.to_owned(),
        }
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
        let mut vec = Vec::from_iter(self.iter_values().map(|v| v.to_string()));
        vec.push(value);
        LinkedListNode::from_vec(vec).unwrap()
    }
    pub fn prepend(&self, value: NodeValue) -> LinkedListNode {
        let mut vec = vec![value];
        vec.extend(self.iter_values().map(|v| v.to_string()));
        LinkedListNode::from_vec(vec).unwrap()
    }
    pub fn clone(&self) -> LinkedListNode {
        LinkedListNode::from_vec(Vec::from_iter(self.iter_values().map(|v| v.to_string()))).unwrap()
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
#[derive(Debug)]
pub struct LinkedListNodeValueIterator<'a> {
    cursor: Option<&'a LinkedListNode>,
}

impl<'a> Iterator for LinkedListNodeIterator<'a> {
    type Item = &'a LinkedListNode;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.cursor?;

        // advance cursor by borrowing the next node
        self.cursor = current.child.as_deref();

        Some(current)
    }
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
            "root".to_string(),
            "child".to_string(),
            "leaf".to_string(),
        ])
        .unwrap();
        let value_list = Vec::from_iter(root.iter_values());
        assert_eq!(value_list, vec!["root", "child", "leaf"])
    }
    #[test]
    fn test_linked_list_node_clone() {
        let root = LinkedListNode::from_vec(vec![
            "root".to_string(),
            "child".to_string(),
            "leaf".to_string(),
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
            child: None,
        };
        let child = LinkedListNode {
            value: NodeValue::from("child"),
            child: Some(Box::from(leaf)),
        };
        let root = LinkedListNode {
            value: NodeValue::from("root"),
            child: Some(Box::from(child)),
        };
        let l_list = Vec::from_iter(root.iter_values());
        assert_eq!(l_list, vec!["root", "child", "leaf"])
    }
}
