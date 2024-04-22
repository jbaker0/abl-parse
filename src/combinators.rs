use winnow::ascii::{alpha1, multispace0, multispace1};
use winnow::combinator::{delimited, separated, trace};
use winnow::error::{ContextError, ErrMode, InputError, ParserError};
use winnow::seq;
use winnow::token::{take_until, take_while};
use winnow::{IResult, PResult, Parser};

use crate::tokens;
use crate::tokens::Include;

static LEFT_GULLWING: char = '{';
static RIGHT_GULLWING: char = '}';
static FULL_STOP: char = '.';
static SPACE: char = ' ';

pub(crate) fn trim<'a, F, O, E>(inner: F) -> impl Parser<&'a str, O, E>
where
    E: ParserError<&'a str>,
    F: Parser<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

pub(crate) fn parse_between_gullwings<'a>(input: &mut &'a str) -> PResult<&'a str> {
    delimited(
        LEFT_GULLWING,
        take_until(0.., RIGHT_GULLWING),
        RIGHT_GULLWING,
    )
    .parse_next(input)
}

pub(crate) fn parse_not_single_digit<'a>(input: &mut &'a str) -> PResult<&'a str> {
    if input.len() == 1 {
        alpha1.parse_next(input)
    } else {
        Ok(input)
    }
}

pub(crate) fn parse_include_path<'a>(input: &mut &'a str) -> PResult<&'a str> {
    // Should use multispace1 instead of SPACE at some point
    trim(take_while(1.., |c| c != SPACE && c != RIGHT_GULLWING )).parse_next(input)
}

// This fails on leading whitespace, may need to fix
pub(crate) fn parse_include_args<'a>(input: &mut &'a str) -> PResult<Vec<&'a str>> {
    let args = separated(0.., 
        take_while(0.., |c: char| !c.is_whitespace() ),
        multispace1
    )
    .parse_next(input)?;

    Ok(args)
}

pub(crate) fn parse_include<'a>(input: &mut &'a str) -> PResult<Include<'a>> {
    let mut between = parse_between_gullwings(input)?;

    let (path, args) = seq!(
        parse_include_path,
        parse_include_args,
    ).parse_next(&mut between)?;

    Ok(Include {
        path,
        content: None,
        arguments: args,
    })
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_between_gullwings() {
        assert_eq!(
            parse_between_gullwings(&mut "{ include.i }"),
            Ok(" include.i ")
        );
        assert_eq!(
            parse_between_gullwings(&mut "{include.i}"),
            Ok("include.i")
        );
        assert_eq!(
            parse_between_gullwings(&mut "{ include.i &val1 &val2 &val3 }"),
            Ok(" include.i &val1 &val2 &val3 ")
        );
        assert_eq!(
            parse_between_gullwings(&mut "{include.i &val1 &val2 &val3}"),
            Ok("include.i &val1 &val2 &val3")
        );
    }

    #[test]
    fn test_parse_include_path() {
        assert_eq!(
            parse_include_path(&mut "include.i a"),
            Ok("include.i")
        );
        assert_eq!(
            parse_include_path(&mut " include.i a"),
            Ok("include.i")
        );
        assert_eq!(
            parse_include_path(&mut "include.i &val1 &val2 &val3"),
            Ok("include.i")
        );
        assert_eq!(
            parse_include_path(&mut " include.i &val1 &val2 &val3 "),
            Ok("include.i")
        );
    }

    #[test]
    fn test_parse_include_args() {
        assert_eq!(
            parse_include_args(&mut "&val1 &val2 &val3"),
            Ok(vec!["&val1", "&val2", "&val3"])
        );
        assert_eq!(
            parse_include_args(&mut "val1 val2 val3 "),
            Ok(vec!["val1", "val2", "val3", ""])
        );
        assert_eq!(
            parse_include_args(&mut "1 2 3"),
            Ok(vec!["1", "2", "3"])
        );
    }

    #[test]
    fn test_parse_include() {
        assert_eq!(
            parse_include(&mut "{include.i}"),
            Ok(Include {
                path: "include.i",
                content: None,
                arguments: vec![""],
            })
        );
        assert_eq!(
            parse_include(&mut "{include.i &val1 &val2 &val3}"),
            Ok(Include {
                path: "include.i",
                content: None,
                arguments: vec!["&val1", "&val2", "&val3"],
            })
        );
    }
}
