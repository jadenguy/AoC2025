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
pub fn connect_junction_boxes_n_times(
    boxes: &Vec<JunctionBox>,
    wanted_connections: i64,
) -> Vec<HashSet<JunctionBox>> {
    connect_junction_boxes(&boxes, wanted_connections).0
}
pub fn connect_junction_boxes_to_exhaustion(boxes: &Vec<JunctionBox>) -> JunctionBoxen {
    connect_junction_boxes(&boxes, -1).1
}
fn connect_junction_boxes(
    boxes: &Vec<JunctionBox>,
    wanted_connections: i64,
) -> (Vec<HashSet<JunctionBox>>, JunctionBoxen) {
    let mut distances: Vec<(JunctionBoxen, i64)> = Vec::new();
    let mut networks: Vec<HashSet<JunctionBox>> = boxes
        .iter()
        .map(|b| HashSet::from_iter([b.clone()]))
        .collect();
    for first_box_index in 0..boxes.len() {
        let a = &boxes[first_box_index];
        // print!("  ");
        // println!("{}: {},{},{}", a.id, a.x, a.y, a.z);
        for second_box_index in (first_box_index + 1)..boxes.len() {
            let b = &boxes[second_box_index];
            distances.push(((a.clone(), b.clone()), a.distance_to_box(b)));
        }
    }
    distances.sort_by_key(|x| x.1);
    // println!("{} pairwise distances found", distances.len());
    let mut connection_count = 0;
    let mut last: Option<JunctionBoxen> = None;
    for ((a, b), _distance) in distances {
        if networks.len() == 1 {
            continue;
        }
        if connection_count == wanted_connections && wanted_connections > 0 {
            continue;
        }
        // print!("  ");
        // print!(
        //     "{} to {} is sqrt({}); ",
        //     a.to_coordinates_string(),
        //     b.to_coordinates_string(),
        //     dist
        // );
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
                // println!("Boxes in one network")
            }
            (Some(mut i), Some(mut j)) => {
                // print!(
                //     "Combining network containing {} and ",
                //     networks[i]
                //         .iter()
                //         .map(|x| x.id.to_string())
                //         .collect::<Vec<_>>()
                //         .join("-")
                // );
                if i > j {
                    (i, j) = (j, i)
                }
                let other = networks.remove(j);
                // println!(
                //     "network containing {}.",
                //     other
                //         .iter()
                //         .map(|x| x.id.to_string())
                //         .collect::<Vec<_>>()
                //         .join("-")
                // );
                // println!();
                networks[i].extend(other);
            }
            _ => {
                panic!("the networks should have been seeded, it's impossible not to find them")
            }
        }
        last = Some((a, b));
        connection_count += 1;
        if connection_count == wanted_connections {
            // println!("");
            // println!(" All connections exhausted.")
        }
    }
    // println!("{} networks found", networks.len());
    networks.sort_by_key(|k| -(k.len() as i64));
    // for net in networks.iter() {
    // print!("  ");
    // print!("{} elements: ", net.len());
    // println!(
    //     "{}",
    //     net.iter()
    //         .map(|x| x.id.to_string())
    //         .collect::<Vec<_>>()
    //         .join("-")
    // )
    // }
    (networks, last.unwrap())
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct JunctionBox {
    pub id: usize,
    pub x: i64,
    pub y: i64,
    pub z: i64,
}
impl JunctionBox {
    // fn to_coordinates_string(&self) -> String {
    //     format!("{},{},{}", self.x, self.y, self.z)
    // }
    fn distance_to_box(&self, other: &JunctionBox) -> i64 {
        let mut dist = 0;
        dist += (self.x - other.x) * (self.x - other.x);
        dist += (self.y - other.y) * (self.y - other.y);
        dist += (self.z - other.z) * (self.z - other.z);
        dist
    }
}
#[cfg(test)]
mod tests {
    use crate::day8::{
        JunctionBox, connect_junction_boxes_n_times, connect_junction_boxes_to_exhaustion,
        parse_junction_boxes,
    };

    #[test]
    fn test_connect_junction_boxes_n_times() {
        // arrange
        let sample_data = sample_data();
        let junction_boxes = parse_junction_boxes(sample_data);
        // act
        let networks = connect_junction_boxes_n_times(&junction_boxes, 10);
        let mut sizes: Vec<usize> = networks.iter().map(|x| x.len()).collect();
        sizes.sort();
        let product_of_three_longest: usize = sizes.iter().rev().take(3).product();
        // assert
        assert_eq!(product_of_three_longest, 40)
    }
    #[test]
    fn test_connect_junction_boxes_to_exhaustion() {
        // arrange
        let sample_data = sample_data();
        let junction_boxes = parse_junction_boxes(sample_data);
        // act
        let networks = connect_junction_boxes_to_exhaustion(&junction_boxes);

        // assert
        assert_eq!(networks.0.x * networks.1.x, 25272)
    }

    #[test]
    fn test_parse_junction_boxes() {
        // arrange
        let sample_data = sample_data();
        let expected = [
            JunctionBox {
                id: 0,
                x: 162,
                y: 817,
                z: 812,
            },
            JunctionBox {
                id: 1,
                x: 57,
                y: 618,
                z: 57,
            },
            JunctionBox {
                id: 2,
                x: 906,
                y: 360,
                z: 560,
            },
            JunctionBox {
                id: 3,
                x: 592,
                y: 479,
                z: 940,
            },
            JunctionBox {
                id: 4,
                x: 352,
                y: 342,
                z: 300,
            },
            JunctionBox {
                id: 5,
                x: 466,
                y: 668,
                z: 158,
            },
            JunctionBox {
                id: 6,
                x: 542,
                y: 29,
                z: 236,
            },
            JunctionBox {
                id: 7,
                x: 431,
                y: 825,
                z: 988,
            },
            JunctionBox {
                id: 8,
                x: 739,
                y: 650,
                z: 466,
            },
            JunctionBox {
                id: 9,
                x: 52,
                y: 470,
                z: 668,
            },
            JunctionBox {
                id: 10,
                x: 216,
                y: 146,
                z: 977,
            },
            JunctionBox {
                id: 11,
                x: 819,
                y: 987,
                z: 18,
            },
            JunctionBox {
                id: 12,
                x: 117,
                y: 168,
                z: 530,
            },
            JunctionBox {
                id: 13,
                x: 805,
                y: 96,
                z: 715,
            },
            JunctionBox {
                id: 14,
                x: 346,
                y: 949,
                z: 466,
            },
            JunctionBox {
                id: 15,
                x: 970,
                y: 615,
                z: 88,
            },
            JunctionBox {
                id: 16,
                x: 941,
                y: 993,
                z: 340,
            },
            JunctionBox {
                id: 17,
                x: 862,
                y: 61,
                z: 35,
            },
            JunctionBox {
                id: 18,
                x: 984,
                y: 92,
                z: 344,
            },
            JunctionBox {
                id: 19,
                x: 425,
                y: 690,
                z: 689,
            },
        ];
        // act
        let junction_boxes = parse_junction_boxes(sample_data);
        // assert
        assert_eq!(junction_boxes, expected);
    }
    fn sample_data() -> Vec<&'static str> {
        r#"162,817,812
            57,618,57
            906,360,560
            592,479,940
            352,342,300
            466,668,158
            542,29,236
            431,825,988
            739,650,466
            52,470,668
            216,146,977
            819,987,18
            117,168,530
            805,96,715
            346,949,466
            970,615,88
            941,993,340
            862,61,35
            984,92,344
            425,690,689"#
            .split("\n")
            .map(|x| x.trim())
            .collect()
    }
}
