use std::{error::Error, fmt, fs::read_to_string, path::PathBuf};

#[derive(Debug)]
struct ApplicationError(&'static str);

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Application error ({})", &self.0)
    }
}

impl Error for ApplicationError {}

fn parse_plan(plan: &str) -> Result<(i64, Vec<Option<i64>>), Box<dyn Error>> {
    let mut plan_lines = plan.lines();
    let time = plan_lines
        .next()
        .ok_or(ApplicationError("missing time"))?
        .parse::<i64>()?;
    let buses = plan_lines
        .next()
        .ok_or(ApplicationError("missing schedule"))?
        .split(",")
        .map(|s| s.parse::<i64>().ok())
        .collect::<Vec<_>>();
    Ok((time, buses))
}

fn part1(time: i64, buses: &[Option<i64>]) -> Option<i64> {
    buses
        .iter()
        .filter_map(|b| b.map(|bus| (bus, bus - time % bus)))
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(bus, wait)| bus * wait)
}

fn modular_inverse(a: i64, m: i64) -> Option<i64> {
    // extended Euclidean algorithm to find inverse a_inv of a modulo m
    // such that given y = x * a (mod m), x = y * a_inv (mod m)
    let (mut t_prev, mut t_curr) = (0, 1);
    let (mut r_prev, mut r_curr) = (m, a);
    while r_curr != 0 {
        let quotient = r_prev / r_curr;
        let t_next = t_prev - quotient * t_curr;
        let r_next = r_prev - quotient * r_curr;
        t_prev = std::mem::replace(&mut t_curr, t_next);
        r_prev = std::mem::replace(&mut r_curr, r_next);
    }

    if r_prev > 1 {
        None
    } else {
        Some(t_prev)
    }
}

fn part2(buses: &[Option<i64>]) -> Option<i64> {
    let am = buses
        .iter()
        .enumerate()
        .filter_map(|(i, b)| b.map(|bus| ((bus - i as i64).rem_euclid(bus), bus)))
        .collect::<Vec<_>>();

    // apply Chinese remainder theorem to equations x ≡ a_i (mod m_i)
    let m_product: i64 = am.iter().map(|(_, m)| *m).product();
    let terms = am.iter().map(|&(a, m)| {
        let n = m_product / m;
        modular_inverse(n, m).map(|y| a * y * n)
    });
    let mut sum = 0;
    for term in terms {
        match term {
            Some(t) => sum += t,
            None => return None,
        }
    }

    Some(sum.rem_euclid(m_product))
}

fn run() -> Result<(), Box<dyn Error>> {
    let path = ["data", "day13", "input.txt"].iter().collect::<PathBuf>();
    let (time, buses) = parse_plan(&read_to_string(path)?)?;
    let result1 = part1(time, &buses).ok_or(ApplicationError("No buses"))?;
    println!("Part1: result = {}", result1);
    let result2 = part2(&buses).ok_or(ApplicationError("Schedule does not permit solution"))?;
    println!("Part2: result = {}", result2);
    Ok(())
}

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("Error occurred: {}", e);
            1
        }
    });
}

#[cfg(test)]
mod test {
    use super::{parse_plan, part1, part2};

    const EXAMPLE: &str = r"939
7,13,x,x,59,x,31,19";

    const EXAMPLE_TIME: i64 = 939;
    const EXAMPLE_BUSES: [Option<i64>; 8] = [
        Some(7),
        Some(13),
        None,
        None,
        Some(59),
        None,
        Some(31),
        Some(19),
    ];

    #[test]
    fn test_parse() {
        let (time, buses) = parse_plan(EXAMPLE).unwrap();
        assert_eq!(time, 939);
        assert_eq!(buses, EXAMPLE_BUSES);
    }

    #[test]
    fn test_part1() {
        let result = part1(EXAMPLE_TIME, &EXAMPLE_BUSES).unwrap();
        assert_eq!(result, 295);
    }

    #[test]
    fn test_part2() {
        let result = part2(&EXAMPLE_BUSES).unwrap();
        assert_eq!(result, 1068781);
    }
}
