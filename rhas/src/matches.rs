
use nom::{IResult, number::complete::double, combinator::{map, recognize, value}, bytes::complete::{tag, is_a}, branch::alt, character::{complete::{char, anychar, one_of}, is_digit, is_alphabetic}, multi::{fold_many0, separated_list0, separated_list1, many0}, sequence::{pair, delimited, tuple}};

use crate::{parser::*};

pub fn number(input: &str) -> IResult<&str, Node> {
    let parser = double;

    map(parser, |val| {
        Node::Number(val)
    })(input)
}

pub fn operator(input: &str) -> IResult<&str, Node> {
    let parser = alt((
        value(Operator::Add, tag("+")),
        value(Operator::Sub, tag("-")),
        value(Operator::Mul, tag("*")),
        value(Operator::Div, tag("/")),
        value(Operator::Pow, tag("^")),
        value(Operator::Fac, tag("!")),
    ));

    map(parser, |op| {
        Node::Operator(op) 
    })(input)
}


pub fn variable(input: &str) -> IResult<&str, Node> {
    //TODO: make this better
    let parser = alt((
        one_of("abcdefghijklmnopqrstuvwxyz"),
        one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
    ));

    map(parser, |ch| {
        Node::Variable(ch.to_string())
    })(input)
}

pub fn identifier(input: &str) -> IResult<&str, &str> {
    //TODO?: make autoloader? match required argument lenght
    let parser = alt((
        tag("sin"),
        tag("cos"),
    ));

    recognize(parser)(input)
}

// fn fenced<'a>(start: &'a str, end: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
//     map(tuple((tag(start), take_until(end), tag(end))), |x| x.1)
// }


pub fn function(input: &str) -> IResult<&str, Node> {
    let name = alt((
        identifier,
    ));

    let parser = tuple((
        identifier,
        delimited(
            char('('), 
            separated_list0(char(','), double), 
            char(')')
        )
    ));

    map(parser, |(name, args)| {
        Node::Function(name.to_string(), args) 
    })(input)
}


pub fn list(input: &str) -> IResult<&str, Node> {
    let parser = delimited(
        char('['), 
        separated_list0(char(','), double), 
        char(']')
    );

    map(parser, |exp| {
        Node::List(exp)
    })(input)
}

pub fn matrix(input: &str) -> IResult<&str, Node> {
    //TODO: add check for matrixes to have even internal list lens
    let parser = delimited(
        tag("#["), 
        separated_list0(char(','), list), 
        char(']')
    );

    map(parser, |mat| {
        Node::Matrix(mat)
    })(input)
}


// pub fn expression(input: &str) -> IResult<>
// // pub fn matrix(input: &str) -> IResult<&str, Node> {
//     // let parser = 
// // }


pub fn parse(input: &str) -> Vec<Node> {
    let (rest, out) = many0(alt((
        list,
        function,
        number,
        operator,
        variable,
    )))(input).unwrap();

    println!("{rest}");

    out
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{parser::*, matches::*};

    #[test]
    fn test_number() {
        assert_eq!(number("43.2"), Ok( ("", Node::Number(43.2)) ));
    }

    #[test]
    fn test_function() {
        println!("{:?}", function("sin()rest"));
        // assert_eq!(function("sin(2,3)").is_ok(), ("rest", Node::Function("sin".to_string(), vec![2.0, 2.0])));

        panic!();
    }

    #[test]
    fn test_list() {
        println!("{:?}", list("[1,3,4,2.322]"));
    }

    #[test]
    fn test_matrix() {
        println!("{:?}", matrix("#[[1,2],[1,3]]"));
    }

    #[test]
    fn test_all() {
        println!("{:?}", parse("x^2+sin(3)+[1,3,4]"));

    }
}