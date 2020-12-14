use std::collections::HashMap;

use crate::common::Instruction;

pub fn execute_program<'a>(program: impl Iterator<Item = &'a Instruction>) -> u64 {
    let mut memory = HashMap::new();
    let mut or_mask = 0;
    let mut and_mask = u64::MAX;
    let mut float_masks = Vec::with_capacity(36);
    for instruction in program {
        match instruction {
            Instruction::Mask(_, ones, floating) => {
                or_mask = *ones;
                and_mask = !floating;
                float_masks.clear();
                (0..36)
                    .map(|i| 1 << i)
                    .filter(|m| floating & m != 0)
                    .for_each(|m| float_masks.push(m));
            }
            Instruction::Assign(base_address, value) => {
                let masked_base = (base_address | or_mask) & and_mask;
                for i in 0..(1usize << float_masks.len()) {
                    let address = float_masks
                        .iter()
                        .enumerate()
                        .filter_map(|(j, m)| if i & (1 << j) == 0 { None } else { Some(*m) })
                        .fold(masked_base, |acc, next| acc | next);
                    memory.insert(address, *value);
                }
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
        Instruction::Mask(!0b110011, 0b10010, 0b100001),
        Instruction::Assign(42, 100),
        Instruction::Mask(!0b1011, 0, 0b1011),
        Instruction::Assign(26, 1),
    ];

    #[test]
    fn execute_test() {
        let result = execute_program(EXAMPLE_PROGRAM.iter());
        assert_eq!(result, 208);
    }
}
