use nom::{error::{ParseError, ErrorKind}, InputTakeAtPosition, IResult, InputIter, InputTake, ToUsize};

use nom::Err;

use crate::matches;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Mod, // %
    Pow, // ^
    Fac, // !
}


#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Operator(Operator),
    Variable(String),
    Number(f64),
    Function(String, Vec<f64>),
    List(Vec<f64>)
}

// pub fn take_while<F, Input, Error: ParseError<Input>>(
//     cond: F,
//   ) -> impl Fn(Input) -> IResult<Input, Input, Error>
//   where
//     Input: InputTakeAtPosition,
//     F: Fn(<Input as InputTakeAtPosition>::Item) -> bool,
//   {
//     move |i: Input| i.split_at_position_complete(|c| !cond(c))
//   }

//   pub fn take<C, Input, Error: ParseError<Input>>(
//     count: C,
//   ) -> impl Fn(Input) -> IResult<Input, Input, Error>
//   where
//     Input: InputIter + InputTake,
//     C: ToUsize,
//   {
//     let c = count.to_usize();
//     move |i: Input| match i.slice_index(c) {
//       Err(_needed) => Err(Err::Error(Error::from_error_kind(i, ErrorKind::Eof))),
//       Ok(index) => Ok(i.take_split(index)),
//     }
//   }

// pub fn take_if<F, Input, Error: ParseError<Input>> (
//     test: F,
// ) -> impl Fn(Input) -> IResult<Input, Input, Error>
// where
//     Input: InputIter,
//     F: Fn(<Input>::Item) -> bool,
//     {

//     }