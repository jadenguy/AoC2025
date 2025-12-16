#[cfg(test)]
mod tests {
    use aoc2025::day1::{Dial, apply_and_count_zeroes_clicks_and_final, parse_instructions};

    #[test]
    fn test_count_zero_clicks_and_positions_with_sample_data() {
        let data = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ];
        let lines: Vec<String> = data.iter().map(|s| s.to_string()).collect();
        let instructions = parse_instructions(&lines);
        let dial = Dial { position: 50 };
        let (total_zero_dials, total_clicks, _final_dial) =
            apply_and_count_zeroes_clicks_and_final(instructions, dial);
        assert_eq!(total_zero_dials, 3);
        assert_eq!(total_clicks, 6);
    }
    #[test]
    fn test_count_zero_clicks_and_positions_with_large_move() {
        let data = vec!["R1000"];
        let lines: Vec<String> = data.iter().map(|s| s.to_string()).collect();
        let instructions = parse_instructions(&lines);
        let dial = Dial { position: 50 };
        let (_total_zero_dials, total_clicks, final_dial) =
            apply_and_count_zeroes_clicks_and_final(instructions, dial);
        assert_eq!(total_clicks, 10);
        assert_eq!(final_dial.position, 50);
    }
    #[test]
    fn test_count_zero_clicks_and_positions_with_left_move_from_zero() {
        let data = vec!["L1"];
        let lines: Vec<String> = data.iter().map(|s| s.to_string()).collect();
        let instructions = parse_instructions(&lines);
        let dial = Dial { position: 0 };
        let (_total_zero_dials, total_clicks, final_dial) =
            apply_and_count_zeroes_clicks_and_final(instructions, dial);
        assert_eq!(total_clicks, 0);
        assert_eq!(final_dial.position, 99);
    }
    #[test]
    fn test_count_zero_clicks_and_positions_with_right_move_from_zero() {
        let data = vec!["R1"];
        let lines: Vec<String> = data.iter().map(|s| s.to_string()).collect();
        let instructions = parse_instructions(&lines);
        let dial = Dial { position: 0 };
        let (_total_zero_dials, total_clicks, final_dial) =
            apply_and_count_zeroes_clicks_and_final(instructions, dial);
        assert_eq!(total_clicks, 0);
        assert_eq!(final_dial.position, 1);
    }
    #[test]
    fn test_count_zero_clicks_and_positions_with_left_move_to_zero() {
        let data = vec!["L1"];
        let lines: Vec<String> = data.iter().map(|s| s.to_string()).collect();
        let instructions = parse_instructions(&lines);
        let dial = Dial { position: 1 };
        let (_total_zero_dials, total_clicks, final_dial) =
            apply_and_count_zeroes_clicks_and_final(instructions, dial);
        assert_eq!(total_clicks, 1);
        assert_eq!(final_dial.position, 0);
    }
    #[test]
    fn test_count_zero_clicks_and_positions_with_right_move_to_zero() {
        let data = vec!["R1"];
        let lines: Vec<String> = data.iter().map(|s| s.to_string()).collect();
        let instructions = parse_instructions(&lines);
        let dial = Dial { position: 99 };
        let (_total_zero_dials, total_clicks, final_dial) =
            apply_and_count_zeroes_clicks_and_final(instructions, dial);
        assert_eq!(total_clicks, 1);
        assert_eq!(final_dial.position, 0);
    }
    #[test]
    fn test_missed_click() {
        let data = vec!["L101"];
        let lines: Vec<String> = data.iter().map(|s| s.to_string()).collect();
        let instructions = parse_instructions(&lines);
        let dial = Dial { position: 1 };
        let (_total_zero_dials, total_clicks, final_dial) =
            apply_and_count_zeroes_clicks_and_final(instructions, dial);
        assert_eq!(total_clicks, 2);
        assert_eq!(final_dial.position, 0);
    }
}

