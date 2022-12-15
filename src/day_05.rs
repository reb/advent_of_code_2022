/// --- Day 5: Supply Stacks ---
///
/// The expedition can depart as soon as the final supplies have been unloaded from the ships.
/// Supplies are stored in stacks of marked crates, but because the needed supplies are buried under
/// many other crates, the crates need to be rearranged.
///
/// The ship has a giant cargo crane capable of moving crates between stacks. To ensure none of the
/// crates get crushed or fall over, the crane operator will rearrange them in a series of
/// carefully-planned steps. After the crates are rearranged, the desired crates will be at the top
/// of each stack.
///
/// The Elves don't want to interrupt the crane operator during this delicate procedure, but they
/// forgot to ask her which crate will end up where, and they want to be ready to unload them as
/// soon as possible so they can embark.
///
/// They do, however, have a drawing of the starting stacks of crates and the rearrangement
/// procedure (your puzzle input). For example:
///
///     [D]
/// [N] [C]
/// [Z] [M] [P]
///  1   2   3
///
/// move 1 from 2 to 1
/// move 3 from 1 to 3
/// move 2 from 2 to 1
/// move 1 from 1 to 2
///
/// In this example, there are three stacks of crates. Stack 1 contains two crates: crate Z is on
/// the bottom, and crate N is on top. Stack 2 contains three crates; from bottom to top, they are
/// crates M, C, and D. Finally, stack 3 contains a single crate, P.
///
/// Then, the rearrangement procedure is given. In each step of the procedure, a quantity of crates
/// is moved from one stack to a different stack. In the first step of the above rearrangement
/// procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:
///
/// [D]
/// [N] [C]
/// [Z] [M] [P]
///  1   2   3
///
/// In the second step, three crates are moved from stack 1 to stack 3. Crates are moved one at a
/// time, so the first crate to be moved (D) ends up below the second and third crates:
///
///         [Z]
///         [N]
///     [C] [D]
///     [M] [P]
///  1   2   3
///
/// Then, both crates are moved from stack 2 to stack 1. Again, because crates are moved one at a
/// time, crate C ends up below crate M:
///
///         [Z]
///         [N]
/// [M]     [D]
/// [C]     [P]
///  1   2   3
///
/// Finally, one crate is moved from stack 1 to stack 2:
///
///         [Z]
///         [N]
///         [D]
/// [C] [M] [P]
///  1   2   3
///
/// The Elves just need to know which crate will end up on top of each stack; in this example, the
/// top crates are C in stack 1, M in stack 2, and Z in stack 3, so you should combine these
/// together and give the Elves the message CMZ.
///
/// After the rearrangement procedure completes, what crate ends up on top of each stack?
use lazy_static::lazy_static;
use regex::Regex;
use std::num::ParseIntError;
use std::str::FromStr;

const INPUT: &str = include_str!("../input/day_05");

pub fn run() {
    let (stacks, instructions) = load_input(INPUT);
}

type Stack = Vec<char>;

#[derive(Debug, PartialEq)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, PartialEq)]
enum ParseInstructionError {
    ParseInt(ParseIntError),
    Regex(String),
}

fn load_input(input: &str) -> (Vec<Stack>, Vec<Instruction>) {
    let mut input_iter = input.split("\n\n");

    let Some(stacks_input) = input_iter.next() else {
        panic!("There was no stack input")
    };
    let stacks = load_stacks(stacks_input);

    let Some(instructions_input) = input_iter.next() else {
        panic!("There was no instructions input")
    };
    let instructions = instructions_input
        .lines()
        .map(Instruction::from_str)
        .filter_map(Result::ok)
        .collect();

    (stacks, instructions)
}

fn load_stacks(input: &str) -> Vec<Stack> {
    let mut stacks = input
        .lines()
        .flat_map(|line| {
            line.chars().enumerate().filter_map(|(i, c)| {
                if c == ' ' || c == '[' || c == ']' {
                    None
                } else {
                    Some((i, c))
                }
            })
        })
        .fold(Vec::new(), |mut vec, (position, c)| {
            let index = position / 4;
            if vec.len() <= index {
                vec.resize(index + 1, Vec::new());
            }
            let inner_vec = vec.get_mut(index).expect("The vec wasn't resized properly");
            inner_vec.insert(0, c);
            vec
        });

    for stack in stacks.iter_mut() {
        // remove the index indicators
        stack.remove(0);
    }
    stacks
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref LINE_EXPRESSION: Regex =
                Regex::new(r"^move (\d) from (\d) to (\d)$").unwrap();
        }
        let captures = LINE_EXPRESSION
            .captures(line)
            .and_then(|cap| Some((cap.get(1), cap.get(2), cap.get(3))));
        match captures {
            Some((Some(amount), Some(from), Some(to))) => {
                match Instruction::parse_str(amount.as_str(), from.as_str(), to.as_str()) {
                    Err(e) => Err(ParseInstructionError::ParseInt(e)),
                    Ok(i) => Ok(i),
                }
            }
            _ => Err(ParseInstructionError::Regex("Couldn't match regex".into())),
        }
    }
}

impl Instruction {
    fn parse_str(amount: &str, from: &str, to: &str) -> Result<Instruction, ParseIntError> {
        Ok(Instruction {
            amount: amount.parse()?,
            from: from.parse()?,
            to: to.parse()?,
        })
    }

    fn apply(&self, mut stacks: Vec<Stack>) -> Vec<Stack> {
        for _ in 0..self.amount {
            // moving a marked crate from the 'from' to the 'to' stack
            let marked_crate = stacks[self.from - 1]
                .pop()
                .expect("There was no crate left in the stack");
            stacks[self.to - 1].push(marked_crate);
        }
        stacks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_stacks() {
        let input = "     [D]\n [N] [C]\n [Z] [M] [P]\n  1   2   3";

        let expected = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];

        assert_eq!(load_stacks(input), expected);
    }

    #[test]
    fn test_instruction_from_str() {
        let input = "move 1 from 2 to 1";

        let expected = Instruction {
            amount: 1,
            from: 2,
            to: 1,
        };

        assert_eq!(Instruction::from_str(input), Ok(expected))
    }

    #[test]
    fn test_apply_instruction_1() {
        //     [D]
        // [N] [C]
        // [Z] [M] [P]
        //  1   2   3
        let input = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];

        // move 1 from 2 to 1
        let instruction = Instruction {
            amount: 1,
            from: 2,
            to: 1,
        };

        // [D]
        // [N] [C]
        // [Z] [M] [P]
        //  1   2   3
        let expected = vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']];

        assert_eq!(instruction.apply(input), expected);
    }

    #[test]
    fn test_apply_instruction_2() {
        // [D]
        // [N] [C]
        // [Z] [M] [P]
        //  1   2   3
        let input = vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']];

        // move 3 from 1 to 3
        let instruction = Instruction {
            amount: 3,
            from: 1,
            to: 3,
        };

        //         [Z]
        //         [N]
        //     [C] [D]
        //     [M] [P]
        //  1   2   3
        let expected = vec![vec![], vec!['M', 'C'], vec!['P', 'D', 'N', 'Z']];

        assert_eq!(instruction.apply(input), expected);
    }

    #[test]
    fn test_apply_instruction_3() {
        //         [Z]
        //         [N]
        //     [C] [D]
        //     [M] [P]
        //  1   2   3
        let input = vec![vec![], vec!['M', 'C'], vec!['P', 'D', 'N', 'Z']];

        // move 2 from 2 to 1
        let instruction = Instruction {
            amount: 2,
            from: 2,
            to: 1,
        };

        //         [Z]
        //         [N]
        // [M]     [D]
        // [C]     [P]
        //  1   2   3
        let expected = vec![vec!['C', 'M'], vec![], vec!['P', 'D', 'N', 'Z']];

        assert_eq!(instruction.apply(input), expected);
    }

    #[test]
    fn test_apply_instruction_4() {
        //         [Z]
        //         [N]
        // [M]     [D]
        // [C]     [P]
        //  1   2   3
        let input = vec![vec!['C', 'M'], vec![], vec!['P', 'D', 'N', 'Z']];

        // move 1 from 1 to 2
        let instruction = Instruction {
            amount: 1,
            from: 1,
            to: 2,
        };

        //         [Z]
        //         [N]
        //         [D]
        // [C] [M] [P]
        //  1   2   3
        let expected = vec![vec!['C'], vec!['M'], vec!['P', 'D', 'N', 'Z']];

        assert_eq!(instruction.apply(input), expected);
    }
}
