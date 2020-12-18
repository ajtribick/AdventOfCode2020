use nom::{character::complete::digit1, combinator::map_res, IResult};
use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error: {}", self.0)
    }
}

impl Error for ParseError {}

fn number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

mod part1 {
    use super::{number, ParseError};

    use nom::{
        branch::alt,
        character::complete::{char, multispace0, one_of},
        combinator::{all_consuming, map},
        multi::fold_many0,
        sequence::tuple,
        Finish, IResult,
    };

    enum Operator {
        Add,
        Multiply,
    }

    fn operator(input: &str) -> IResult<&str, Operator> {
        map(one_of("+*"), |c| match c {
            '+' => Operator::Add,
            '*' => Operator::Multiply,
            _ => unreachable!(),
        })(input)
    }

    fn unary_expr(input: &str) -> IResult<&str, u64> {
        alt((number, bracket_expr))(input)
    }

    fn bracket_expr(input: &str) -> IResult<&str, u64> {
        map(tuple((char('('), operator_expr, char(')'))), |(_, x, _)| x)(input)
    }

    fn operator_expr(input: &str) -> IResult<&str, u64> {
        let (second, initial) = unary_expr(input)?;
        fold_many0(
            tuple((multispace0, operator, multispace0, unary_expr)),
            initial,
            |acc, (_, op, _, next)| match op {
                Operator::Add => acc + next,
                Operator::Multiply => acc * next,
            },
        )(second)
    }

    pub fn expr(input: &str) -> Result<u64, ParseError> {
        all_consuming(operator_expr)(input)
            .finish()
            .map_or_else(|e| Err(ParseError(e.to_string())), |(_, v)| Ok(v))
    }
}

mod part2 {
    use super::{number, ParseError};

    use nom::{
        branch::alt,
        character::complete::{char, multispace0},
        combinator::{all_consuming, map},
        multi::fold_many0,
        sequence::tuple,
        Finish, IResult,
    };

    pub fn unary_expr(input: &str) -> IResult<&str, u64> {
        alt((number, bracket_expr))(input)
    }

    fn bracket_expr(input: &str) -> IResult<&str, u64> {
        map(tuple((char('('), operator_expr, char(')'))), |(_, x, _)| x)(input)
    }

    fn add_expr(input: &str) -> IResult<&str, u64> {
        let (rhs, initial) = unary_expr(input)?;
        fold_many0(
            tuple((multispace0, char('+'), multispace0, unary_expr)),
            initial,
            |acc, next| acc + next.3,
        )(rhs)
    }

    pub fn operator_expr(input: &str) -> IResult<&str, u64> {
        let (rhs, initial) = add_expr(input)?;
        fold_many0(
            tuple((multispace0, char('*'), multispace0, add_expr)),
            initial,
            |acc, next| acc * next.3,
        )(rhs)
    }

    pub fn expr(input: &str) -> Result<u64, ParseError> {
        all_consuming(operator_expr)(input)
            .finish()
            .map_or_else(|e| Err(ParseError(e.to_string())), |(_, v)| Ok(v))
    }
}

pub use part1::expr;
pub use part2::expr as expr_advanced;

#[cfg(test)]
mod test {
    use super::{expr, expr_advanced};

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
        for &(src, expected, _) in EXAMPLES.iter() {
            let result = expr(src).unwrap();
            assert_eq!(result, expected, "Failed on {}", src);
        }
    }

    #[test]
    fn expr_advanced_test() {
        for &(src, _, expected) in EXAMPLES.iter() {
            println!("{}", src);
            let result = expr_advanced(src).unwrap();
            assert_eq!(result, expected, "Failed on {}", src);
        }
    }
}
