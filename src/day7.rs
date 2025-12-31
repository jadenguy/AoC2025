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
pub fn process_manifold(manifold: &Manifold) -> (usize, usize) {
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
#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::day7::{parse_manifold_strings, process_manifold};

    #[test]
    fn test_parse_manifold_strings() {
        // arrange
        let sample_data = sample_data();
        // act
        let actual_manifold = parse_manifold_strings(sample_data);
        // assert
        let first_emitter: (usize, usize) = *actual_manifold.emitters.iter().nth(0).unwrap();
        let split: HashSet<(usize, usize)> = actual_manifold.splitters;
        assert_eq!(first_emitter, (0, 7));
        assert_eq!(
            split,
            HashSet::from_iter([
                (02, 07),
                (04, 06),
                (04, 08),
                (06, 05),
                (06, 07),
                (06, 09),
                (08, 04),
                (08, 06),
                (08, 10),
                (10, 03),
                (10, 05),
                (10, 09),
                (10, 11),
                (12, 02),
                (12, 06),
                (12, 12),
                (14, 01),
                (14, 03),
                (14, 05),
                (14, 07),
                (14, 09),
                (14, 13),
            ])
        );
    }
    #[test]
    fn test_process_manifold() {
        // arrange
        let sample_data = sample_data();
        let actual_manifold = parse_manifold_strings(sample_data);
        // act
        let processed_output = process_manifold(&actual_manifold).0;
        // assert
        assert_eq!(processed_output, 21);
    }
    #[test]
    fn test_process_quantum_manifold() {
        // arrange
        let sample_data = sample_data();
        let actual_manifold = parse_manifold_strings(sample_data);
        // act
        let processed_output = process_manifold(&actual_manifold).1;
        // assert
        assert_eq!(processed_output, 40);
    }
    fn sample_data() -> Vec<&'static str> {
        r#".......S.......
           ...............
           .......^.......
           ...............
           ......^.^......
           ...............
           .....^.^.^.....
           ...............
           ....^.^...^....
           ...............
           ...^.^...^.^...
           ...............
           ..^...^.....^..
           ...............
           .^.^.^.^.^...^.
           ..............."#
            .split("\n")
            .map(|x| x.trim())
            .collect()
    }
}
