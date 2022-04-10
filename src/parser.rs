use nom::character::complete::multispace0;
use nom::error::ParseError;
use nom::sequence::delimited;
use nom::{
    branch::alt,
    character::complete::{char, digit1},
    combinator::{map, recognize},
    sequence::tuple,
    IResult,
};

type ParserResult<'a> = IResult<&'a str, Expr>;

#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Add,
    Mul,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Num(f64),
    Neg(Box<Expr>),
    BinOp {
        op: Op,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}

impl Eq for Expr {}

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn int(input: &str) -> ParserResult {
    map(digit1, |s: &str| Expr::Num(s.parse::<f64>().unwrap()))(input)
}

fn float(input: &str) -> ParserResult {
    map(recognize(tuple((digit1, char('.'), digit1))), |s: &str| {
        Expr::Num(s.parse::<f64>().unwrap())
    })(input)
}

fn num(input: &str) -> ParserResult {
    // FIXME: This accepts "4." as well, and leaves a "." in stream
    alt((float, int))(input)
}

fn add(input: &str) -> ParserResult {
    map(tuple((num, ws(char('+')), num)), |(l, _o, r)| Expr::BinOp {
        op: Op::Add,
        lhs: Box::new(l),
        rhs: Box::new(r),
    })(input)
}

pub fn parse(input: &str) -> ParserResult {
    ws(alt((add, num)))(input)
}

#[cfg(test)]
mod test {
    use nom::error::{Error, ErrorKind};

    use super::{int, Expr};

    #[test]
    fn test_int() {
        // Success cases
        assert_eq!(int("3"), Ok(("", Expr::Num(3.0))));
        assert_eq!(int("3321"), Ok(("", Expr::Num(3321.0))));
        assert_eq!(int("33.21"), Ok((".21", Expr::Num(33.0))));
        // Failure cases
        assert_eq!(
            int("s"),
            Err(nom::Err::Error(Error {
                input: "s",
                code: ErrorKind::Digit
            }))
        );
    }
}
