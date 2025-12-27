use std::collections::HashSet;

// type Manifold = HashMap<Coordinate, char>;
type Coordinate = (usize, usize);
type CoordinateSet = HashSet<Coordinate>;
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
pub fn process_manifold_with_gravity(manifold: Manifold) -> (Vec<Coordinate>, usize) {
    let mut beams: CoordinateSet = manifold.emitters.clone();
    let mut split = 0;
    let mut next_row = CoordinateSet::new();
    for row_num in 0..manifold.height {
        next_row = CoordinateSet::new();
        for col_num in 0..(manifold.width - 1) {
            let coord = (row_num, col_num);
            let coord_below = (row_num + 1, col_num);

            if beams.contains(&coord) {
                if manifold.splitters.contains(&coord_below) {
                    split += 1;
                    if col_num > 0 {
                        next_row.insert((row_num + 1, col_num - 1));
                    }
                    if col_num < manifold.width {
                        next_row.insert((row_num + 1, col_num + 1));
                    }
                } else {
                    next_row.insert(coord_below);
                }
            }
            if row_num < manifold.height {
                next_row.iter().for_each(|b| {
                    beams.insert(b.to_owned());
                });
            }
        }
    }
    (next_row.iter().map(|c| *c).collect(), split)
}
pub struct Manifold {
    pub emitters: CoordinateSet,
    pub splitters: CoordinateSet,
    pub width: usize,
    pub height: usize,
}
