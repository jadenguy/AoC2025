use std::iter;

type Indicator = u8;
type ButtonDefinition = Vec<usize>;
type Joltage = i64;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct MachineDefinition {
    wanted_indicator_state: Vec<Indicator>,
    buttons: Vec<ButtonDefinition>,
    wanted_joltage: Vec<Joltage>,
}

pub fn parse_machine_instructions(d: &str) -> MachineDefinition {
    enum ParseState {
        Indicator,
        Button,
        Jolt,
    }
    let mut wanted_indicator: Vec<Indicator> = Vec::new();
    let mut buttons: Vec<ButtonDefinition> = Vec::new();
    let mut joltage: Vec<Joltage> = Vec::new();
    let mut parse_state = ParseState::Indicator;
    let mut buffer: String = String::from("");
    let mut button_definition_buffer: ButtonDefinition = Vec::new();
    for c in d.chars() {
        match (c, &parse_state) {
            (']', _) => {
                parse_state = ParseState::Button;
                buffer.clear();
            }
            ('{', _) => {
                parse_state = ParseState::Jolt;
                buffer.clear();
            }
            ('.', ParseState::Indicator) => {
                wanted_indicator.push(0);
            }
            ('#', ParseState::Indicator) => {
                wanted_indicator.push(1);
            }
            ('(', ParseState::Button) => {
                buffer.clear();
            }
            (',', ParseState::Button) => {
                button_definition_buffer.push(buffer.parse().unwrap());
                buffer.clear();
            }
            (')', ParseState::Button) => {
                button_definition_buffer.push(buffer.parse().unwrap());
                buttons.push(button_definition_buffer.clone());
                button_definition_buffer.clear();
            }
            ('}', ParseState::Jolt) | (',', ParseState::Jolt) => {
                joltage.push(buffer.parse().unwrap());
                buffer.clear();
            }
            _ => {
                buffer.push(c);
            }
        }
    }
    MachineDefinition {
        wanted_indicator_state: wanted_indicator,
        buttons,
        wanted_joltage: joltage,
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct MachineState {
    pub lit_indicators: Vec<Indicator>,
    pub active_joltage: Vec<Joltage>,
    pub instructions: MachineDefinition,
    pub pushes: Vec<usize>,
}

impl MachineState {
    pub fn push_button(&self, button_index: &usize) -> MachineState {
        let instructions = self.instructions.to_owned();
        let mut lit_indicators = self.lit_indicators.to_owned();
        let mut active_joltage = self.active_joltage.to_owned();
        let mut pushes = self.pushes.to_owned();
        pushes[*button_index] += 1;
        let items = &instructions.buttons[*button_index];
        push_botton(items, &mut lit_indicators, &mut active_joltage);
        MachineState {
            lit_indicators,
            instructions,
            active_joltage,
            pushes: pushes,
        }
    }
    pub fn is_on(&self) -> bool {
        self.lit_indicators
            .iter()
            .eq(self.instructions.wanted_indicator_state.iter())
    }
    pub fn from_instructions(instructions: &MachineDefinition) -> MachineState {
        let indicator_count = instructions.wanted_indicator_state.len();
        MachineState {
            instructions: instructions.to_owned(),
            lit_indicators: iter::repeat(0).take(indicator_count).collect(),
            active_joltage: iter::repeat(0).take(indicator_count).collect(),
            pushes: iter::repeat(0).take(instructions.buttons.len()).collect(),
        }
    }

    fn joltage_diffs(&self) -> Vec<i64> {
        self.instructions
            .wanted_joltage
            .iter()
            .zip(self.active_joltage.iter())
            .map(|(a, b)| a - b)
            .collect()
    }
}

fn push_botton(items: &[usize], new_indicators: &mut [Indicator], active_joltage: &mut [Joltage]) {
    for light_index in items {
        new_indicators[*light_index] = (1 + new_indicators[*light_index]) % 2;
        active_joltage[*light_index] += 1;
    }
}
pub fn find_min_presses_for_indicators(m: &MachineState) -> usize {
    let len = m.instructions.buttons.len();
    let max_button_state: usize = 1 << (len);
    let mut min_presses = len;
    for button_press_state in 0..=max_button_state {
        let buttons_pressed: Vec<usize> = (0..len)
            .filter_map(|indicator_index| {
                let var_name = button_press_state >> indicator_index;
                if 1 == ((var_name) & 1) {
                    return Some(indicator_index);
                }
                None
            })
            .collect();
        let x = buttons_pressed
            .iter()
            .fold(m.to_owned(), |state, &button| state.push_button(&button));
        let is_solved = x.is_on();
        if buttons_pressed.len() < min_presses && is_solved {
            // fun_name(m, button_press_state, &buttons_pressed, &x);
            min_presses = buttons_pressed.len();
        }
    }
    min_presses
}
pub fn find_minimal_presses_for_joltage(initial_state: &MachineState) -> usize {
    let mut unchecked_states: Vec<MachineState> = vec![initial_state.clone()];
    let mut checked_states: Vec<MachineState> = Vec::new();
    let mut solutions: Vec<MachineState> = Vec::new();
    let button_indexes: Vec<usize> = (0..initial_state.instructions.buttons.len()).collect();
    for button_index in button_indexes {
        while let Some(state) = unchecked_states.pop() {
            let mut keep_going = true;
            let mut new_state: MachineState = state.to_owned();
            while keep_going {
                let tnew_state = new_state.push_button(&button_index);
                new_state = tnew_state.clone();
                let list_of_joltages_needed = (new_state).joltage_diffs();
                if list_of_joltages_needed.iter().any(|&d| d < 0) {
                    keep_going = false;
                } else if list_of_joltages_needed.iter().all(|&d| d == 0) {
                    solutions.push(tnew_state)
                } else {
                    checked_states.push(tnew_state);
                }
            }
            checked_states.push(state);
        }

        unchecked_states = checked_states.to_owned();
        checked_states.clear();
    }
    solutions
        .iter()
        .map(|s| s.pushes.iter().sum())
        .min()
        .unwrap()
    // return 0;

    // todo!(
    //     r#"find_minimal_presses_for_joltage:
    // create a vec of accumulated button presses where the joltage is under the target
    //     press each button and add it to the stack.
    //     then compare the finished results for the lowest score."#
    // );
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_minimal_presses_for_joltage() {
        // arrange
        let machines: Vec<MachineState> = sample_data()
            .iter()
            .map(|d| MachineState::from_instructions(&parse_machine_instructions(d)))
            .collect();
        // act
        let actual: Vec<usize> = machines
            .iter()
            .map(|m| find_minimal_presses_for_joltage(m))
            .collect();
        // assert
        assert_eq!(actual, [10, 12, 11])
    }
    #[test]
    fn test_find_minimal_presses_for_indicators() {
        // arrange
        let machines: Vec<MachineState> = sample_data()
            .iter()
            .map(|d| MachineState::from_instructions(&parse_machine_instructions(d)))
            .collect();
        // act
        let actual: Vec<usize> = machines
            .iter()
            .map(|m| find_min_presses_for_indicators(m))
            .collect();
        // assert
        assert_eq!(actual, [2, 3, 2])
    }
    #[test]
    fn test_push_botton_on_state() {
        let initial = MachineState {
            pushes: vec![0, 0, 0, 0, 0, 0],
            lit_indicators: vec![0, 0, 0, 0],
            active_joltage: vec![0, 0, 0, 0],
            instructions: MachineDefinition {
                wanted_indicator_state: vec![0, 1, 1, 0],
                buttons: vec![
                    vec![3],    //0
                    vec![1, 3], //1
                    vec![2],    //2
                    vec![2, 3], //3
                    vec![0, 2], //4
                    vec![0, 1], //5
                ],
                wanted_joltage: vec![3, 5, 4, 7],
            },
        };
        assert_eq!(initial.push_button(&0).lit_indicators, [0, 0, 0, 1]);

        assert_eq!(initial.push_button(&1).lit_indicators, [0, 1, 0, 1]);

        assert_eq!(
            initial.push_button(&1).push_button(&1).lit_indicators,
            [0, 0, 0, 0]
        );
        assert_eq!(
            initial.push_button(&3).push_button(&5).lit_indicators,
            [1, 1, 1, 1]
        );
        assert_eq!(
            initial.push_button(&0).push_button(&0).active_joltage,
            [0, 0, 0, 2]
        );
    }
    #[test]
    fn test_state_from_instruction() {
        let expected = MachineState {
            pushes: vec![0, 0, 0, 0, 0, 0],
            lit_indicators: vec![0, 0, 0, 0],
            active_joltage: vec![0, 0, 0, 0],
            instructions: MachineDefinition {
                wanted_indicator_state: vec![0, 1, 1, 0],
                buttons: vec![
                    vec![3],
                    vec![1, 3],
                    vec![2],
                    vec![2, 3],
                    vec![0, 2],
                    vec![0, 1],
                ],
                wanted_joltage: vec![3, 5, 4, 7],
            },
        };
        let actual = MachineState::from_instructions(&expected.instructions);
        assert_eq!(actual.lit_indicators, expected.lit_indicators);
    }
    #[test]
    fn test_push_button() {
        let button: &[usize] = &[0, 2];
        let expected_output = [1u8, 1, 0, 0];
        let mut actual_indicators = [0u8, 1, 1, 0];
        let mut actual_joltage = [0, 0, 0, 0];
        push_botton(button, &mut actual_indicators, &mut actual_joltage);
        assert_eq!(actual_indicators, expected_output);
    }
    #[test]
    fn test_parse_tiles() {
        // arrange
        // act
        let actual_instructions: Vec<MachineDefinition> = sample_data()
            .iter()
            .map(|d| parse_machine_instructions(d))
            .collect();
        // assert
        assert_eq!(
            actual_instructions,
            [
                MachineDefinition {
                    wanted_indicator_state: vec![0, 1, 1, 0],
                    buttons: vec![
                        vec![3],
                        vec![1, 3],
                        vec![2],
                        vec![2, 3],
                        vec![0, 2],
                        vec![0, 1]
                    ],
                    wanted_joltage: vec![3, 5, 4, 7]
                },
                MachineDefinition {
                    wanted_indicator_state: vec![0, 0, 0, 1, 0],
                    buttons: vec![
                        vec![0, 2, 3, 4],
                        vec![2, 3],
                        vec![0, 4],
                        vec![0, 1, 2],
                        vec![1, 2, 3, 4]
                    ],
                    wanted_joltage: vec![7, 5, 12, 7, 2]
                },
                MachineDefinition {
                    wanted_indicator_state: vec![0, 1, 1, 1, 0, 1],
                    buttons: vec![
                        vec![0, 1, 2, 3, 4],
                        vec![0, 3, 4],
                        vec![0, 1, 2, 4, 5],
                        vec![1, 2]
                    ],
                    wanted_joltage: vec![10, 11, 11, 5, 10, 5]
                }
            ]
        )
    }
    fn sample_data() -> Vec<&'static str> {
        r#" [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
            [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
            [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#
            .split("\n")
            .map(|x| x.trim())
            .collect()
    }
}
