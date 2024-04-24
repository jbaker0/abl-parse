use winnow::combinator::*;
use winnow::token::any;
use winnow::PResult;
use winnow::Parser;

use crate::combinators::*;
use crate::tokens::*;

mod combinators;
pub mod tokens;

pub fn parse_input<'a>(input: &mut &'a str) -> PResult<Statement<'a>> {
    trim(dispatch! {peek(any);
    '/' => parse_comment,
    '{' => parse_include,
    _ => skip_to_comment_or_include
    })
    .parse_next(input)
}

pub fn parse_procedure<'a>(input: &'a str) -> PResult<Vec<Statement<'a>>> {
    let mut iter = iterator(input, parse_input);
    let statements = iter.collect::<Vec<Statement>>();
    let _result = iter.finish();

    Ok(statements)
}
