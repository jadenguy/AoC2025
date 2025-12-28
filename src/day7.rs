use std::collections::{HashMap, HashSet};

type Coordinate = (usize, usize);
type CoordinateSet = HashSet<Coordinate>;
type BeamCount = HashMap<Coordinate, usize>;
pub fn parse_manifold_strings(data: Vec<&str>) -> Manifold {
    let mut emitters = HashSet::new();
    let mut splitters = HashSet::new();
    let height = data.len();
    let mut width = 0;
    for (row_num, &line) in data.iter().enumerate() {
        width = line.len();
        for (col_num, char) in line.char_indices() {
            let coord = (row_num, col_num);
            if char == 'S' {
                emitters.insert(coord);
            } else if char == '^' {
                splitters.insert(coord);
            }
        }
    }
    Manifold {
        emitters: emitters,
        splitters: splitters,
        width: width,
        height: height,
    }
}
pub fn process_manifold(manifold: Manifold) -> (usize, usize) {
    let mut beam_paths: BeamCount =
        HashMap::from_iter(manifold.emitters.iter().map(|&e| (e, 1usize)));
    let mut splits_hit = CoordinateSet::new();
    let mut next_row: BeamCount = HashMap::new();
    for row_num in 0..manifold.height {
        next_row = HashMap::new();
        for col_num in 0..manifold.width {
            let coord = (row_num, col_num);
            let coord_below = (row_num + 1, col_num);

            if let Some(&beam_count) = beam_paths.get(&coord) {
                // println!("{},{} beams {}", row_num, col_num, beam_count);
                if !manifold.splitters.contains(&coord_below) {
                    if let Some(current_count) = next_row.get(&coord_below) {
                        next_row.insert(coord_below, beam_count + current_count);
                    } else {
                        next_row.insert(coord_below, beam_count);
                    }
                } else {
                    splits_hit.insert(coord);
                    if col_num > 0 {
                        let split_coord_l = (row_num + 1, col_num - 1);
                        if let Some(current_count) = next_row.get(&split_coord_l) {
                            next_row.insert(split_coord_l, beam_count + current_count);
                        } else {
                            next_row.insert(split_coord_l, beam_count);
                        }
                    }
                    if col_num < manifold.width {
                        let split_coord_r = (row_num + 1, col_num + 1);
                        if let Some(current_count) = next_row.get(&split_coord_r) {
                            next_row.insert(split_coord_r, beam_count + current_count);
                        } else {
                            next_row.insert(split_coord_r, beam_count);
                        }
                    }
                }
            }
        }
        if row_num < manifold.height {
            let mut sorted_row: Vec<(Coordinate, usize)> =
                next_row.iter().map(|c| (*c.0, *c.1)).collect();
            sorted_row.sort();
            sorted_row.iter().for_each(|b| {
                // println!("{},{} path count {}", b.0.0, b.0.1, b.1);
                beam_paths.insert(b.0.to_owned(), b.1);
            })
        }
    }
    (
        splits_hit.len(),
        next_row
            .iter()
            .map(|x| {
                // println!("{},{} {}", x.0.0, x.0.1, x.1);
                x.1
            })
            .sum(),
    )
}
pub struct Manifold {
    pub emitters: CoordinateSet,
    pub splitters: CoordinateSet,
    pub width: usize,
    pub height: usize,
}
