use std::collections::HashMap;

use crate::common::Instruction;

pub fn execute_program<'a>(program: impl Iterator<Item = &'a Instruction>) -> u64 {
    let mut memory = HashMap::new();
    let mut or_mask = 0;
    let mut and_mask = u64::MAX;
    for instruction in program {
        match instruction {
            Instruction::Mask(zeroes, ones, _) => {
                or_mask = *ones;
                and_mask = !zeroes;
            }
            Instruction::Assign(address, value) => {
                match (value | or_mask) & and_mask {
                    0 => memory.remove(address),
                    v => memory.insert(*address, v),
                };
            }
        }
    }

    memory.values().sum()
}

#[cfg(test)]
mod test {
    use super::execute_program;

    use crate::common::Instruction;

    const EXAMPLE_PROGRAM: [Instruction; 4] = [
        Instruction::Mask(0b10, 0b1000000, !0b1000010),
        Instruction::Assign(8, 11),
        Instruction::Assign(7, 101),
        Instruction::Assign(8, 0),
    ];

    #[test]
    fn execute_test() {
        let result = execute_program(EXAMPLE_PROGRAM.iter());
        assert_eq!(result, 165);
    }
}
