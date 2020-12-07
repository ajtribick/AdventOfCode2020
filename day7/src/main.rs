use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

mod day7error;
mod rule;

use day7error::Day7Error;
use rule::Rule;

const BAG_TYPE: &str = "shiny gold";

fn part1(rules: &Vec<Rule>) -> usize {
    let mut nodes = HashMap::new();
    for rule in rules.iter() {
        for (_, color) in rule.bag_list() {
            match nodes.get_mut(&color[..]) {
                None => {
                    nodes.insert(&color[..], (false, vec![rule.color()]));
                }
                Some((_, vec)) => vec.push(rule.color()),
            }
        }

        if !nodes.contains_key(rule.color()) {
            nodes.insert(rule.color(), (false, Vec::new()));
        }
    }

    let mut keys = vec![BAG_TYPE];
    while let Some(key) = keys.pop() {
        let node = nodes.get_mut(key).unwrap();
        if node.0 {
            continue;
        }
        node.0 = true;
        node.1.iter().for_each(|k| keys.push(k));
    }

    nodes.values().filter(|(valid, _)| *valid).count() - 1
}

fn count_node(nodes: &HashMap<&str, &[(i32, String)]>, node: &str) -> usize {
    nodes.get(node).unwrap().iter().fold(0, |acc, (n, t)| {
        acc + *n as usize * (1 + count_node(nodes, t))
    })
}

pub fn part2(lines: &Vec<Rule>) -> usize {
    let map = lines.iter().map(|r| (r.color(), r.bag_list())).collect();
    count_node(&map, BAG_TYPE)
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::new();
    ["data", "day7", "input.txt"]
        .iter()
        .for_each(|p| path.push(p));
    let file = File::open(path)?;
    let rules = BufReader::new(file)
        .lines()
        .map(|l| l.map_err(Day7Error::IoError).and_then(|s| s.parse()))
        .collect::<Result<_, _>>()?;

    println!("Part 1: {} valid bags", part1(&rules));
    println!("Part 2: {} contained bags", part2(&rules));

    Ok(())
}

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("Error occurred: {}", e);
            1
        }
    })
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const RULES1: &str = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const RULES2: &str = r"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn part1_test() {
        let rules = RULES1.lines().map(str::parse).map(Result::unwrap).collect();
        let result = part1(&rules);

        assert_eq!(result, 4);
    }

    #[test]
    fn part1_test_rev() {
        let rules = RULES1
            .lines()
            .rev()
            .map(str::parse)
            .map(Result::unwrap)
            .collect();
        let result = part1(&rules);

        assert_eq!(result, 4);
    }

    #[test]
    fn part2_rules1_test() {
        let rules = RULES1.lines().map(str::parse).map(Result::unwrap).collect();
        let result = part2(&rules);
        assert_eq!(result, 32);
    }

    #[test]
    fn part2_rules1_test_rev() {
        let rules = RULES1
            .lines()
            .rev()
            .map(str::parse)
            .map(Result::unwrap)
            .collect();
        let result = part2(&rules);
        assert_eq!(result, 32);
    }

    #[test]
    fn part2_rules2_test() {
        let rules = RULES2.lines().map(str::parse).map(Result::unwrap).collect();
        let result = part2(&rules);
        assert_eq!(result, 126);
    }

    #[test]
    fn part2_rules2_test_rev() {
        let rules = RULES2
            .lines()
            .rev()
            .map(str::parse)
            .map(Result::unwrap)
            .collect();
        let result = part2(&rules);
        assert_eq!(result, 126);
    }
}
