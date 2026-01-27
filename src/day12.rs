pub struct Requirement {
    pub shapes: Vec<usize>,
    pub areas: Vec<((usize, usize), Vec<usize>)>,
}

impl Requirement {
    pub fn from_strings(lines: Vec<String>) -> Requirement {
        Requirement::from_strs(lines.iter().map(|s| s.as_str()).collect())
    }
    pub fn from_strs(lines: Vec<&str>) -> Requirement {
        let shapes: Vec<usize> = Vec::new();
        let mut areas: Vec<((usize, usize), Vec<usize>)> = Vec::with_capacity(1300);
        for line in lines {
            if line.contains("x") {
                let mut line_split: Vec<&str> = line.split(" ").collect();
                let area_chars = line_split.remove(0);
                let mut number: String = String::new();
                let mut area = (0usize, 0usize);
                for char in area_chars.chars() {
                    match char {
                        'x' => {
                            area = (number.parse().unwrap(), 0);
                            number = String::new();
                        }
                        ':' => {
                            area = (area.0, number.parse::<usize>().unwrap());
                            number = String::new();
                        }
                        _ => number.push(char),
                    }
                }
                let tiles: Vec<usize> = line_split
                    .iter()
                    .map(|number| number.parse::<usize>().unwrap())
                    .collect();
                areas.push(((area), tiles));
            }
        }
        Requirement { shapes, areas }
    }
    pub fn always_possible_areas(&self) -> Vec<usize> {
        fn fun_name((index, row): (usize, &((usize, usize), Vec<usize>))) -> Option<usize> {
            let (width, height) = row.0;
            let area = (width / 3) * (height / 3);
            let items = &row.1;
            println!("{:?}", items);
            let tiles = items.iter().sum();
            let valid = area >= tiles;
            if valid { return Some(index) } else { None }
        }
        self.areas.iter().enumerate().filter_map(fun_name).collect()
    }
}
#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_parse_reactor() {
        let lines = sample_data();
        let req = Requirement::from_strs(lines);
        let mut iter = req.areas.iter();
        assert_eq!(iter.next(), Some(&((4, 4), vec![0, 0, 0, 0, 2, 0])));
        assert_eq!(iter.next(), Some(&((12, 5), vec![1, 0, 1, 0, 2, 2])));
        assert_eq!(iter.next(), Some(&((12, 5), vec![1, 0, 1, 0, 3, 2])));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn test_always_passes() {
        let lines = sample_data();
        let req = Requirement::from_strs(lines);
        assert_eq!(req.always_possible_areas().len(), 0)
    }

    pub fn sample_data() -> Vec<&'static str> {
        r#" 0:
            ###
            ##.
            ##.

            1:
            ###
            ##.
            .##

            2:
            .##
            ###
            ##.

            3:
            ##.
            ###
            ##.

            4:
            ###
            #..
            ###

            5:
            ###
            .#.
            ###

            4x4: 0 0 0 0 2 0
            12x5: 1 0 1 0 2 2
            12x5: 1 0 1 0 3 2"#
            .split("\n")
            .map(|x| x.trim())
            .collect()
    }
}
