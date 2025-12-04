use std::fs::File;
use std::io::{self, BufRead, BufReader};

struct Instruction {
    distance: i32,
}

struct Dial {
    position: i32,
}

impl Dial {
    fn apply_instruction(&self, instruction: &Instruction) -> Dial {
        Dial {
            position: ((self.position + instruction.distance) % 100 + 100) % 100,
        }
    }
}
impl Instruction {
    fn from_str(s: &str) -> Option<Instruction> {
        let (dir, mag_str) = s.split_at(1);
        let magnitude: i32 = mag_str.parse().ok()?;
        let distance = match dir {
            "L" => -magnitude,
            "R" => magnitude,
            _ => return None,
        };
        Some(Instruction { distance: distance })
    }
}
fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}
fn count_zero_positions(instructions: Vec<Instruction>, dial: Dial) -> i32 {
    let mut zero_position_count = 0;
    let mut current_dial = Dial {
        position: dial.position,
    };

    if current_dial.position == 0 {
        zero_position_count += 1;
    }
    for instruction in instructions {
        current_dial = current_dial.apply_instruction(&instruction);
        if current_dial.position == 0 {
            zero_position_count += 1;
        }
    }
    zero_position_count
}
fn parse_instructions(lines: &Vec<String>) -> Vec<Instruction> {
    lines
        .iter()
        .filter_map(|line| Instruction::from_str(line))
        .collect()
}
fn main() {
    let lines = read_lines("./part1.txt").expect("Failed to read lines from file");
    let instructions = parse_instructions(&lines);
    let dial = Dial { position: 50 };
    let zero_position_count = count_zero_positions(instructions, dial);
    println!("Part 1: Your password is {}", zero_position_count);
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_instruction_from_str_left() {
        let instr = super::Instruction::from_str("L99");
        assert!(instr.is_some());
        let instr = instr.unwrap();
        assert_eq!(instr.distance, -99);
    }

    #[test]
    fn test_instruction_from_str_right() {
        let instr = super::Instruction::from_str("R42");
        assert!(instr.is_some());
        let instr = instr.unwrap();
        assert_eq!(instr.distance, 42);
    }

    #[test]
    fn test_instruction_from_str_invalid() {
        let instr = super::Instruction::from_str("X10");
        assert!(instr.is_none());
        let instr = super::Instruction::from_str("Lxx");
        assert!(instr.is_none());
        let instr = super::Instruction::from_str("");
        assert!(instr.is_none());
    }

    #[test]
    fn test_apply_instruction_wraps_correctly() {
        let dial = super::Dial { position: 95 };
        let instr = super::Instruction { distance: 10 };
        let new_dial = dial.apply_instruction(&instr);
        assert_eq!(new_dial.position, 5);

        let dial = super::Dial { position: 5 };
        let instr = super::Instruction { distance: -10 };
        let new_dial = dial.apply_instruction(&instr);
        assert_eq!(new_dial.position, 95);
    }

    #[test]
    fn test_count_zero_positions_with_sample_data() {
        let data = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ];
        let lines: Vec<String> = data.iter().map(|s| s.to_string()).collect();
        let instructions = super::parse_instructions(&lines);
        let dial = super::Dial { position: 50 };
        let zero_count = super::count_zero_positions(instructions, dial);
        assert_eq!(zero_count, 3); // Adjust this expected value if needed
    }
}
