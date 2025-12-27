use std::collections::HashMap;

// type Manifold = HashMap<Coordinate, char>;
type Coordinate = (i32, i32);

pub fn parse_manifold_strings(data: Vec<&str>) -> Manifold {
    let mut emitters = HashMap::new();
    let mut splitters = HashMap::new();
    let height = data.len();
    for (row_num, &line) in data.iter().enumerate() {
        for (col_num, char) in line.char_indices() {
            let coord = (row_num as i32, col_num as i32);
            if char == 'S' {
                emitters.insert(coord, char);
            } else if char == '^' {
                splitters.insert(coord, char);
            }
        }
    }
    Manifold {
        emitters: emitters,
        splitters: splitters,
        height: height,
    }
}
pub fn process_manifold(manifold: Manifold) -> Vec<Coordinate> {
    vec![]
}
pub struct Manifold {
    pub emitters: HashMap<Coordinate, char>,
    pub splitters: HashMap<Coordinate, char>,
    height: usize,
}
