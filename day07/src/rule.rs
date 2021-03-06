use std::{error, fmt, str::FromStr};

#[derive(Debug)]
pub struct ParseRuleError {}

impl fmt::Display for ParseRuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse rule")
    }
}

impl error::Error for ParseRuleError {}

#[derive(Debug, PartialEq)]
pub struct Rule {
    color: String,
    bag_list: Vec<(i32, String)>,
}

impl Rule {
    pub fn color(&self) -> &str {
        &self.color
    }

    pub fn bag_list(&self) -> &[(i32, String)] {
        &self.bag_list
    }
}

mod parse {
    use super::Rule;

    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, char, digit1},
        combinator::{map, map_res, opt, recognize},
        multi::separated_list1,
        sequence::{separated_pair, terminated, tuple},
        IResult,
    };

    fn color(input: &str) -> IResult<&str, String> {
        map(
            recognize(separated_pair(alpha1, char(' '), alpha1)),
            str::to_owned,
        )(input)
    }

    fn bag_list_entry(input: &str) -> IResult<&str, (i32, String)> {
        terminated(
            separated_pair(map_res(digit1, str::parse), char(' '), color),
            tuple((tag(" bag"), opt(char('s')))),
        )(input)
    }

    fn bag_list(input: &str) -> IResult<&str, Vec<(i32, String)>> {
        separated_list1(tag(", "), bag_list_entry)(input)
    }

    fn no_bags(input: &str) -> IResult<&str, Vec<(i32, String)>> {
        map(tag("no other bags"), |_| Vec::new())(input)
    }

    pub fn rule(input: &str) -> IResult<&str, Rule> {
        map(
            terminated(
                separated_pair(color, tag(" bags contain "), alt((no_bags, bag_list))),
                char('.'),
            ),
            |(color, bag_list)| Rule { color, bag_list },
        )(input)
    }
}

impl FromStr for Rule {
    type Err = ParseRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse::rule(s).map_or(Err(ParseRuleError {}), |(_, r)| Ok(r))
    }
}

#[cfg(test)]
mod test {
    use super::Rule;

    #[test]
    fn rule_test() {
        let rule_text = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        let expected = [
            ("light red", vec![(1, "bright white"), (2, "muted yellow")]),
            (
                "dark orange",
                vec![(3, "bright white"), (4, "muted yellow")],
            ),
            ("bright white", vec![(1, "shiny gold")]),
            ("muted yellow", vec![(2, "shiny gold"), (9, "faded blue")]),
            ("shiny gold", vec![(1, "dark olive"), (2, "vibrant plum")]),
            ("dark olive", vec![(3, "faded blue"), (4, "dotted black")]),
            ("vibrant plum", vec![(5, "faded blue"), (6, "dotted black")]),
            ("faded blue", vec![]),
            ("dotted black", vec![]),
        ]
        .iter()
        .map(|(c, v)| Rule {
            color: (*c).to_owned(),
            bag_list: v.iter().map(|&(n, bc)| (n, bc.to_owned())).collect(),
        })
        .collect::<Vec<_>>();

        let parsed = rule_text
            .lines()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>();

        assert_eq!(expected, parsed);
    }
}
