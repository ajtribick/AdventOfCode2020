use std::cmp::Ordering;

pub fn sqrt_exact(count: usize) -> Option<usize> {
    for size in 1usize.. {
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
