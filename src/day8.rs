use std::collections::HashSet;
type JunctionBoxen = (JunctionBox, JunctionBox);
pub fn parse_junction_boxes(sample_data: Vec<&str>) -> Vec<JunctionBox> {
    sample_data
        .iter()
        .enumerate()
        .map(|(i, &r)| {
            let mut itr = r.split(",");
            JunctionBox {
                id: i,
                x: itr.next().unwrap().parse().unwrap(),
                y: itr.next().unwrap().parse().unwrap(),
                z: itr.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}
pub fn connect_junction_boxes(
    boxes: Vec<JunctionBox>,
    connection_count: i64,
) -> Vec<HashSet<JunctionBox>> {
    let mut distances: Vec<(JunctionBoxen, i64)> = Vec::new();
    let mut networks: Vec<HashSet<JunctionBox>> = boxes
        .iter()
        .map(|b| HashSet::from_iter([b.clone()]))
        .collect();
    for first_box_index in 0..boxes.len() {
        let a = &boxes[first_box_index];
        print!("  ");
        println!("{}: {},{},{}", a.id, a.x, a.y, a.z);
        for second_box_index in (first_box_index + 1)..boxes.len() {
            let b = &boxes[second_box_index];
            distances.push(((a.clone(), b.clone()), a.distance_to_box(b)));
        }
    }
    distances.sort_by_key(|x| x.1);
    println!("{} pairwise distances found", distances.len());
    let mut connections = 0;
    for ((a, b), dist) in distances {
        if connections == connection_count {
            continue;
        }
        print!("  ");
        print!("{} to {} is sqrt({}); ", a.id, b.id, dist);
        let mut net_a = None;
        let mut net_b = None;
        for (idx, net) in networks.iter().enumerate() {
            if net.contains(&a) {
                net_a = Some(idx);
            }
            if net.contains(&b) {
                net_b = Some(idx);
            }
        }
        match (net_a, net_b) {
            (Some(i), Some(j)) if i == j => {
                println!("Boxes in one network")
            }
            (Some(i), Some(j)) => {
                print!(
                    "Combining network containing {} and ",
                    networks[i]
                        .iter()
                        .map(|x| x.id.to_string())
                        .collect::<Vec<_>>()
                        .join("-")
                );
                let other = networks.remove(j);
                println!(
                    "network containing {}.",
                    other
                        .iter()
                        .map(|x| x.id.to_string())
                        .collect::<Vec<_>>()
                        .join("-")
                );
                networks[i].extend(other);
            }
            _ => {
                panic!("the networks should have been seeded, it's impossible not to find them")
            }
        }
        connections += 1;
        if connections == connection_count {
            println!("");
            println!(" All connections exhausted.")
        }
    }
    println!("{} networks found", networks.len());
    networks.sort_by_key(|k| 1000 - k.len());
    for net in networks.iter() {
        print!("  ");
        print!("{} elements: ", net.len());
        println!(
            "{}",
            net.iter()
                .map(|x| x.id.to_string())
                .collect::<Vec<_>>()
                .join("-")
        )
    }
    networks
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct JunctionBox {
    pub id: usize,
    pub x: i64,
    pub y: i64,
    pub z: i64,
}
impl JunctionBox {
    fn distance_to_box(&self, other: &JunctionBox) -> i64 {
        let mut dist = 0;
        dist += (self.x - other.x) * (self.x - other.x);
        dist += (self.y - other.y) * (self.y - other.y);
        dist += (self.z - other.z) * (self.z - other.z);
        dist
    }
}
