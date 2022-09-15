use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::combinator::{map, value};
use nom::IResult;
use nom::multi::many0;
use nom::number::complete::double;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Operator(Operator),
    Identifier(String),
    Number(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

fn parse_op(input: &str) -> IResult<&str, Token> {
    let op = alt((
        value(Operator::Add, tag("+")),
        value(Operator::Sub, tag("-")),
        value(Operator::Mul, tag("*")),
        value(Operator::Div, tag("/")),
    ));
    map(op, |res| Token::Operator(res))(input)
}

fn parse_num(input: &str) -> IResult<&str, Token> {
    let parser = |s| double(s);
    map(parser, |res| Token::Number(res))(input)
}

fn parse_id(input: &str) -> IResult<&str, Token> {
    let parser = |s| alpha1(s);
    map(parser, |res| Token::Identifier(String::from(res)))(input)
}

fn parse_global(input: &str) -> IResult<&str, Token> {
    alt((parse_op, parse_id, parse_num))(input)
}

pub fn parse(input: &str) -> Vec<Token> {
    let result = many0(parse_global)(input);

    result.unwrap().1
}


#[test]
fn test() {
    assert_eq!(parse("x+3+3x"), vec![
        Token::Identifier(String::from("x")),
        Token::Operator(Operator::Add),
        Token::Number(3.0),
        Token::Operator(Operator::Add),
        Token::Number(3.0),
        Token::Identifier(String::from("x"))
    ]);
}