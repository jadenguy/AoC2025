// --- Day 10: Factory ---

// Just across the hall, you find a large factory. Fortunately, the Elves here have plenty of time to decorate. Unfortunately, it's because the factory machines are all offline, and none of the Elves can figure out the initialization procedure.

// The Elves do have the manual for the machines, but the section detailing the initialization procedure was eaten by a Shiba Inu. All that remains of the manual are some indicator light diagrams, button wiring schematics, and joltage requirements for each machine.

// For example:

// [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
// [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
// [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}

use std::iter;

// The manual describes one machine per line. Each line contains a single indicator light diagram in [square brackets], one or more button wiring schematics in (parentheses), and joltage requirements in {curly braces}.
type Indicator = u8;
type ButtonDefinition = Vec<usize>;
type Joltage = usize;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct MachineDefinition {
    wanted_indicator_state: Vec<Indicator>,
    buttons: Vec<ButtonDefinition>,
    joltage: Vec<Joltage>,
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
    let mut current_buttons: ButtonDefinition = Vec::new();
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
                current_buttons.push(buffer.parse().unwrap());
                buffer.clear();
            }
            (')', ParseState::Button) => {
                current_buttons.push(buffer.parse().unwrap());
                buttons.push(current_buttons.clone());
                current_buttons.clear();
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
        joltage,
    }
}
// To start a machine, its indicator lights must match those shown in the diagram, where . means off and # means on. The machine has the number of indicator lights shown, but its indicator lights are all initially off.

// So, an indicator light diagram like [.##.] means that the machine has four indicator lights which are initially off and that the goal is to simultaneously configure the first light to be off, the second light to be on, the third to be on, and the fourth to be off.

// You can toggle the state of indicator lights by pushing any of the listed buttons. Each button lists which indicator lights it toggles, where 0 means the first light, 1 means the second light, and so on. When you push a button, each listed indicator light either turns on (if it was off) or turns off (if it was on). You have to push each button an integer number of times; there's no such thing as "0.5 presses" (nor can you push a button a negative number of times).

// So, a button wiring schematic like (0,3,4) means that each time you push that button, the first, fourth, and fifth indicator lights would all toggle between on and off. If the indicator lights were [#.....], pushing the button would change them to be [...##.] instead.

// Because none of the machines are running, the joltage requirements are irrelevant and can be safely ignored.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MachineState {
    pub lit_indicators: Vec<Indicator>,
    instructions: MachineDefinition,
}
impl MachineState {
    pub fn push_button(&self, button_index: usize) -> MachineState {
        let instructions = self.instructions.to_owned();
        let mut new_indicators = self.lit_indicators.to_owned();
        let items = &instructions.buttons[button_index];
        push_botton(&mut new_indicators, items);
        MachineState {
            lit_indicators: new_indicators,
            instructions,
        }
    }
    pub fn is_on(&self) -> bool {
        self.lit_indicators
            .iter()
            .eq(self.instructions.wanted_indicator_state.iter())
    }
    pub fn from_instructions(instructions: &MachineDefinition) -> MachineState {
        let instructions = instructions.to_owned();
        let n = instructions.wanted_indicator_state.len();
        MachineState {
            instructions,
            lit_indicators: iter::repeat(0).take(n).collect(),
        }
    }
}

fn push_botton(new_indicators: &mut [u8], items: &[usize]) {
    for light_index in items {
        new_indicators[*light_index] = (1 + new_indicators[*light_index]) % 2;
    }
}

// You can push each button as many times as you like. However, to save on time, you will need to determine the fewest total presses required to correctly configure all indicator lights for all machines in your list.

fn find_min_presses(m: &MachineState) -> usize {
    todo!()
}

// There are a few ways to correctly configure the first machine:

// [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
// You could press the first three buttons once each, a total of 3 button presses.
// You could press (1,3) once, (2,3) once, and (0,1) twice, a total of 4 button presses.
// You could press all of the buttons except (1,3) once each, a total of 5 button presses.
// However, the fewest button presses required is 2. One way to do this is by pressing the last two buttons ((0,2) and (0,1)) once each.

// The second machine can be configured with as few as 3 button presses:

// [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
// One way to achieve this is by pressing the last three buttons ((0,4), (0,1,2), and (1,2,3,4)) once each.

// The third machine has a total of six indicator lights that need to be configured correctly:

// [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
// The fewest presses required to correctly configure it is 2; one way to do this is by pressing buttons (0,3,4) and (0,1,2,4,5) once each.

// So, the fewest button presses required to correctly configure the indicator lights on all of the machines is 2 + 3 + 2 = 7.

// Analyze each machine's indicator light diagram and button wiring schematics. What is the fewest button presses required to correctly configure the indicator lights on all of the machines?

// To begin, get your puzzle input.

// Answer:

// You can also [Share] this puzzle.
pub fn x() {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_minimal_presses() {
        // arrange
        let machines: Vec<MachineState> = sample_data()
            .iter()
            .map(|d| MachineState::from_instructions(&parse_machine_instructions(d)))
            .collect();
        // act
        let actual: Vec<usize> = machines.iter().map(|m| find_min_presses(m)).collect();
        // assert
        assert_eq!(actual, [2, 3, 2])
    }
    #[test]
    fn test_push_botton_on_state() {
        let initial = MachineState {
            lit_indicators: vec![0, 0, 0, 0],
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
                joltage: vec![3, 5, 4, 7],
            },
        };
        assert_eq!(initial.push_button(0).lit_indicators, [0, 0, 0, 1]);

        assert_eq!(initial.push_button(1).lit_indicators, [0, 1, 0, 1]);

        assert_eq!(
            initial.push_button(1).push_button(1).lit_indicators,
            [0, 0, 0, 0]
        );
        assert_eq!(
            initial.push_button(3).push_button(5).lit_indicators,
            [1, 1, 1, 1]
        );
    }
    #[test]
    fn test_state_from_instruction() {
        let expected = MachineState {
            lit_indicators: vec![0, 0, 0, 0],
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
                joltage: vec![3, 5, 4, 7],
            },
        };
        let actual = MachineState::from_instructions(&expected.instructions);
        assert_eq!(actual.lit_indicators, expected.lit_indicators);
    }
    #[test]
    fn test_push_button() {
        let button: &[usize] = &[0, 2];
        let expected_output = [1u8, 1, 0, 0];
        let mut actual_output = [0u8, 1, 1, 0];
        push_botton(&mut actual_output, button);
        assert_eq!(actual_output, expected_output);
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
                    joltage: vec![3, 5, 4, 7]
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
                    joltage: vec![7, 5, 12, 7, 2]
                },
                MachineDefinition {
                    wanted_indicator_state: vec![0, 1, 1, 1, 0, 1],
                    buttons: vec![
                        vec![0, 1, 2, 3, 4],
                        vec![0, 3, 4],
                        vec![0, 1, 2, 4, 5],
                        vec![1, 2]
                    ],
                    joltage: vec![10, 11, 11, 5, 10, 5]
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
