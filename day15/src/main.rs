use ahash::AHashMap;

fn elf_sequence(initial: &[usize], n: usize) -> usize {
    assert!(!initial.is_empty());
    let mut memory = initial
        .iter()
        .enumerate()
        .map(|(i, x)| (*x, i + 1))
        .collect::<AHashMap<_, _>>();
    let mut item = *initial.last().unwrap();
    for pos in initial.len()..n {
        let mut next_item = 0;
        memory
            .entry(item)
            .and_modify(|prev_pos| {
                next_item = pos - *prev_pos;
                *prev_pos = pos;
            })
            .or_insert(pos);

        item = next_item;
    }

    item
}

const INPUT: [usize; 6] = [1, 0, 16, 5, 17, 4];

fn main() {
    println!("Part 1: result = {}", elf_sequence(&INPUT, 2020));
    println!("Part 2: result = {}", elf_sequence(&INPUT, 30000000));
}

#[cfg(test)]
mod test {
    use super::elf_sequence;

    #[test]
    fn part1_test() {
        const TESTS: [([usize; 3], usize); 7] = [
            ([0, 3, 6], 436),
            ([1, 3, 2], 1),
            ([2, 1, 3], 10),
            ([1, 2, 3], 27),
            ([2, 3, 1], 78),
            ([3, 2, 1], 438),
            ([3, 1, 2], 1836),
        ];

        for (sequence, expected) in TESTS.iter() {
            let result = elf_sequence(sequence, 2020);
            assert_eq!(result, *expected);
        }
    }

    // To save time, run each part 2 test as its own test case (enabling
    // cargo test to run them in parallel), and only on optimized builds.
    macro_rules! part2_test {
        ($name:ident, $seq:expr, $expected:expr) => {
            #[test]
            #[cfg(not(debug_assertions))]
            fn $name() {
                let result = elf_sequence(&$seq, 30000000);
                assert_eq!(result, $expected);
            }
        };
    }

    part2_test!(part2_test1, [0, 3, 6], 175594);
    part2_test!(part2_test2, [1, 3, 2], 2578);
    part2_test!(part2_test3, [2, 1, 3], 3544142);
    part2_test!(part2_test4, [1, 2, 3], 261214);
    part2_test!(part2_test5, [2, 3, 1], 6895259);
    part2_test!(part2_test6, [3, 2, 1], 18);
    part2_test!(part2_test7, [3, 1, 2], 362);
}
