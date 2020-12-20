use std::cmp::Ordering;

pub fn sqrt_exact(count: usize) -> Option<usize> {
    for size in 0usize.. {
        if let Some(sqr_size) = size.checked_mul(size) {
            match sqr_size.cmp(&count) {
                Ordering::Less => (),
                Ordering::Equal => return Some(size),
                Ordering::Greater => return None,
            }
        } else {
            return None;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    use super::sqrt_exact;

    #[test]
    fn sqrt_exact_solve_test() {
        for expected in 0..100 {
            let value = expected * expected;
            assert_eq!(sqrt_exact(value), Some(expected))
        }
    }

    #[test]
    fn sqrt_exact_no_result_test() {
        let squares = (0..100).map(|i| i * i).collect::<Vec<_>>();
        for i in 0..*squares.last().unwrap() {
            if squares.contains(&i) {
                continue;
            }

            assert_eq!(sqrt_exact(i), None)
        }
    }
}
