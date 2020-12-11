use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1},
    combinator::{map, map_res, opt, recognize},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

use crate::day7error::Day7Error;

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

fn color(input: &str) -> IResult<&str, &str> {
    recognize(separated_pair(alpha1, char(' '), alpha1))(input)
}

fn bag_list_entry(input: &str) -> IResult<&str, (i32, &str)> {
    map(
        tuple((
            map_res(digit1, str::parse),
            char(' '),
            color,
            tag(" bag"),
            opt(char('s')),
        )),
        |(quantity, _, color_text, _, _)| (quantity, color_text),
    )(input)
}

fn bag_list(input: &str) -> IResult<&str, Vec<(i32, &str)>> {
    separated_list1(tag(", "), bag_list_entry)(input)
}

fn no_bags(input: &str) -> IResult<&str, Vec<(i32, &str)>> {
    map(tag("no other bags"), |_| Vec::new())(input)
}

fn rule(input: &str) -> IResult<&str, Rule> {
    map(
        tuple((
            color,
            tag(" bags contain "),
            alt((no_bags, bag_list)),
            char('.'),
        )),
        |(c, _, bl, _)| Rule {
            color: c.to_owned(),
            bag_list: bl.iter().map(|&(n, bc)| (n, bc.to_owned())).collect(),
        },
    )(input)
}

impl FromStr for Rule {
    type Err = Day7Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        rule(s).map_or(Err(Day7Error::ParseError), |(_, r)| Ok(r))
    }
}

#[cfg(test)]
mod test {
    use super::Rule;

    use std::error::Error;

    #[test]
    fn rule_test() -> Result<(), Box<dyn Error>> {
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
            .map(str::parse)
            .collect::<Result<Vec<Rule>, _>>()?;

        assert_eq!(expected, parsed);

        Ok(())
    }
}
