pub struct Instruction {
    pub distance: i32,
}
pub struct Dial {
    pub position: i32,
}
struct DialPositionUpdate {
    dial: Dial,
    clicks: i32,
}
impl Dial {
    fn apply_instruction_and_listen(&self, instruction: &Instruction) -> DialPositionUpdate {
        let target_position = self.position + instruction.distance;
        let total_clicks_into_zero = self.count_clicks_into_zero(target_position);
        let normalized_dial_position = ((target_position) % 100 + 100) % 100;
        DialPositionUpdate {
            dial: Dial {
                position: normalized_dial_position,
            },
            clicks: total_clicks_into_zero,
        }
    }

    fn count_clicks_into_zero(&self, target_position: i32) -> i32 {
        // Count the number of times the dial goes to 0 from the left or right
        //   in the course of following the instruction.
        if self.position > 0 && target_position <= 0 {
            return target_position.abs() / 100 + 1;
        } else {
            return target_position.abs() / 100;
        }
    }
}
impl Instruction {
    fn from_str(s: &str) -> Option<Instruction> {
        if s.len() < 2 {
            return None;
        }
        let (dir, mag_str) = s.split_at(1);
        let magnitude: i32 = mag_str.parse().ok()?;
        let distance = match dir {
            "L" => -magnitude,
            "R" => magnitude,
            _ => return None,
        };
        Some(Instruction { distance: distance })
    }
    fn to_str(inst: &Instruction) -> String {
        let mut direction = "R";
        if inst.distance < 0 {
            direction = "L";
        }
        format!("{}{}", direction, inst.distance.abs())
    }
}
pub fn parse_instructions(lines: &Vec<String>) -> Vec<Instruction> {
    lines
        .iter()
        .filter_map(|line| Instruction::from_str(line))
        .collect()
}
fn get_results_after_instructions(
    dial: Dial,
    instructions: Vec<Instruction>,
    verbose: bool,
) -> Vec<DialPositionUpdate> {
    let mut results = Vec::new();
    let mut current_dial = dial;
    let mut running_total_clicks = 0;
    if verbose {
        println!("The dial starts by pointing at {}.", current_dial.position);
    }
    for instruction in instructions {
        let update = current_dial.apply_instruction_and_listen(&instruction);
        if verbose {
            print!(
                "The dial is rotated {} to point at {}; ",
                Instruction::to_str(&instruction),
                update.dial.position
            );
        }
        if update.clicks > 0 {
            if verbose {
                print!(
                    "during this rotation, it points at zero {} times; ",
                    update.clicks
                );
            }
            running_total_clicks += update.clicks;
        }
        if verbose {
            println!("the running total is {}.", running_total_clicks);
        }
        current_dial = Dial {
            position: update.dial.position,
        };
        results.push(update);
    }
    results
}

pub fn apply_and_count_zeroes_clicks_and_final(
    instructions: Vec<Instruction>,
    dial_initial_position: i32,
    verbose: bool,
) -> (i32, i32, Dial) {
    let results = get_results_after_instructions(
        Dial {
            position: dial_initial_position,
        },
        instructions,
        verbose,
    );
    let total_zero_dials: i32 = results
        .iter()
        .filter(|update| update.dial.position == 0)
        .count() as i32;
    let total_clicks: i32 = results.iter().map(|update| update.clicks).sum();
    let final_dial = Dial {
        position: results.last().unwrap().dial.position,
    };
    (total_zero_dials, total_clicks, final_dial)
}

#[cfg(test)]
mod tests {
    use crate::day1::{apply_and_count_zeroes_clicks_and_final, parse_instructions};

    #[test]
    fn test_count_zero_clicks_and_positions_with_sample_data() {
        // arrange
        let data = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ];
        let lines: Vec<String> = data.iter().map(|s| s.to_string()).collect();
        let instructions = parse_instructions(&lines);
        let dial = 50;
        // act
        let (total_zero_dials, total_clicks, _final_dial) =
            apply_and_count_zeroes_clicks_and_final(instructions, dial, false);
        // assert
        assert_eq!(total_zero_dials, 3);
        assert_eq!(total_clicks, 6);
    }
    #[test]
    fn test_count_zero_clicks_and_positions_with_large_move() {
        // arrange
        let data = vec!["R1000"];
        let lines: Vec<String> = data.iter().map(|s| s.to_string()).collect();
        let instructions = parse_instructions(&lines);
        let dial = 50;
        // act
        let (_total_zero_dials, total_clicks, final_dial) =
            apply_and_count_zeroes_clicks_and_final(instructions, dial, false);
        // assert
        assert_eq!(total_clicks, 10);
        assert_eq!(final_dial.position, 50);
    }
    #[test]
    fn test_count_zero_clicks_and_positions_with_left_move_from_zero() {
        // arrange
        let data = vec!["L1"];
        let lines: Vec<String> = data.iter().map(|s| s.to_string()).collect();
        let instructions = parse_instructions(&lines);
        let dial = 0;
        // act
        let (_total_zero_dials, total_clicks, final_dial) =
            apply_and_count_zeroes_clicks_and_final(instructions, dial, false);
        // assert
        assert_eq!(total_clicks, 0);
        assert_eq!(final_dial.position, 99);
    }
    #[test]
    fn test_count_zero_clicks_and_positions_with_right_move_from_zero() {
        // arrange

        let data = vec!["R1"];
        let lines: Vec<String> = data.iter().map(|s| s.to_string()).collect();
        let instructions = parse_instructions(&lines);
        let dial = 0;
        // act
        let (_total_zero_dials, total_clicks, final_dial) =
            apply_and_count_zeroes_clicks_and_final(instructions, dial, false);
        // assert
        assert_eq!(total_clicks, 0);
        assert_eq!(final_dial.position, 1);
    }
    #[test]
    fn test_count_zero_clicks_and_positions_with_left_move_to_zero() {
        // arrange
        let data = vec!["L1"];
        let lines: Vec<String> = data.iter().map(|s| s.to_string()).collect();
        let instructions = parse_instructions(&lines);
        let dial = 1;
        // act
        let (_total_zero_dials, total_clicks, final_dial) =
            apply_and_count_zeroes_clicks_and_final(instructions, dial, false);
        // assert
        assert_eq!(total_clicks, 1);
        assert_eq!(final_dial.position, 0);
    }
    #[test]
    fn test_count_zero_clicks_and_positions_with_right_move_to_zero() {
        // arrange
        let data = vec!["R1"];
        let lines: Vec<String> = data.iter().map(|s| s.to_string()).collect();
        let instructions = parse_instructions(&lines);
        let dial = 99;
        // act

        let (_total_zero_dials, total_clicks, final_dial) =
            apply_and_count_zeroes_clicks_and_final(instructions, dial, false);
        // assert
        assert_eq!(total_clicks, 1);
        assert_eq!(final_dial.position, 0);
    }
    #[test]
    fn test_missed_click() {
        // arrange
        let data = vec!["L101"];
        let lines: Vec<String> = data.iter().map(|s| s.to_string()).collect();
        let instructions = parse_instructions(&lines);
        let dial = 1;
        // act
        let (_total_zero_dials, total_clicks, final_dial) =
            apply_and_count_zeroes_clicks_and_final(instructions, dial, false);
        // assert
        assert_eq!(total_clicks, 2);
        assert_eq!(final_dial.position, 0);
    }
}
