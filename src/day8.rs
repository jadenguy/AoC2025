use std::collections::HashSet;

pub fn parse_junction_boxes(sample_data: Vec<&str>) -> Vec<Box> {
    sample_data
        .iter()
        .map(|&r| {
            let mut itr = r.split(",");
            Box {
                x: itr.next().unwrap().parse().unwrap(),
                y: itr.next().unwrap().parse().unwrap(),
                z: itr.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}
pub fn connect_junction_boxes(boxes: Vec<Box>, connections: usize) -> Vec<HashSet<Box>> {
    vec![]
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Box {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
