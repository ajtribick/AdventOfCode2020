use std::{error::Error, fmt};

use nom::{
    branch::alt,
    character::complete::{char, digit1, multispace0, one_of},
    combinator::{all_consuming, map, map_res},
    multi::fold_many0,
    sequence::{delimited, preceded, tuple},
    Finish, IResult,
};

#[derive(Debug)]
pub struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error: {}", self.0)
    }
}

impl Error for ParseError {}

trait Parser {
    fn expr(s: &str) -> IResult<&str, u64>;
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

fn number(s: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(s)
}

fn operator(s: &str) -> IResult<&str, Operator> {
    map(one_of("+*"), |c| match c {
        '+' => Operator::Add,
        '*' => Operator::Multiply,
        _ => unreachable!(),
    })(s)
}

fn bracket_expr<P: Parser>(s: &str) -> IResult<&str, u64> {
    delimited(char('('), P::expr, char(')'))(s)
}

fn unary_expr<P: Parser>(s: &str) -> IResult<&str, u64> {
    alt((number, bracket_expr::<P>))(s)
}

struct SimpleParser {}

impl Parser for SimpleParser {
    fn expr(s: &str) -> IResult<&str, u64> {
        let (rhs, initial) = unary_expr::<Self>(s)?;
        fold_many0(
            tuple((
                preceded(multispace0, operator),
                preceded(multispace0, unary_expr::<Self>),
            )),
            initial,
            |acc, (op, next)| match op {
                Operator::Add => acc + next,
                Operator::Multiply => acc * next,
            },
        )(rhs)
    }
}

struct AdvancedParser {}

impl AdvancedParser {
    fn add_expr(s: &str) -> IResult<&str, u64> {
        let (rhs, initial) = unary_expr::<Self>(s)?;
        fold_many0(
            preceded(
                tuple((multispace0, char('+'), multispace0)),
                unary_expr::<Self>,
            ),
            initial,
            |acc, next| acc + next,
        )(rhs)
    }
}

impl Parser for AdvancedParser {
    fn expr(s: &str) -> IResult<&str, u64> {
        let (rhs, initial) = Self::add_expr(s)?;
        fold_many0(
            preceded(tuple((multispace0, char('*'), multispace0)), Self::add_expr),
            initial,
            |acc, next| acc * next,
        )(rhs)
    }
}

pub fn parse(s: &str, use_precedence: bool) -> Result<u64, ParseError> {
    let expr = if use_precedence {
        AdvancedParser::expr
    } else {
        SimpleParser::expr
    };

    all_consuming(expr)(s)
        .finish()
        .map_or_else(|e| Err(ParseError(e.to_string())), |(_, v)| Ok(v))
}

#[cfg(test)]
mod test {
    use super::parse;

    const EXAMPLES: [(&str, u64, u64); 6] = [
        ("1 + 2 * 3 + 4 * 5 + 6", 71, 231),
        ("1 + (2 * 3) + (4 * (5 + 6))", 51, 51),
        ("2 * 3 + (4 * 5)", 26, 46),
        ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437, 1445),
        ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240, 669060),
        (
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
            13632,
            23340,
        ),
    ];

    #[test]
    fn expr_test() {
        for &(src, expected, _) in &EXAMPLES {
            let result = parse(src, false).unwrap();
            assert_eq!(result, expected, "Failed on {}", src);
        }
    }

    #[test]
    fn expr_advanced_test() {
        for &(src, _, expected) in &EXAMPLES {
            let result = parse(src, true).unwrap();
            assert_eq!(result, expected, "Failed on {}", src);
        }
    }
}
