pub struct Requirement {
    shapes: Vec<u8>,
    areas: Vec<(u8, Vec<u8>)>,
}

impl Requirement {
    pub fn from_strings(lines: Vec<String>) -> Requirement {
        // enum State {
        //     shapes,
        //     areas,
        // }
        // let mut state: State = State::shapes;
        // let shapeSize = 0;
        let shapes = Vec::new();
        let areas = Vec::new();
        for line in lines {
            if line.contains(":") {
                let mut line_split: Vec<&str> = line.split(" ").collect();
                let area_chars = line_split.remove(0);
                let mut number: String = String::new();
                let mut area = 0u8;
                for char in area_chars.chars() {
                    match char {
                        'x' => {
                            area = number.parse().unwrap();
                            number = String::new();
                        }
                        ':' => {
                            area *= number.parse::<u8>().unwrap();
                            number = String::new();
                        }
                        _ => number.push(char),
                    }
                }
            }
        }
        Requirement { shapes, areas }
    }
}
#[cfg(test)]
pub mod tests {
    use super::*;

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
