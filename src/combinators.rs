use winnow::ascii::{alpha1, multispace0, multispace1};
use winnow::combinator::{alt, delimited, repeat_till, separated};
use winnow::error::ParserError;
use winnow::seq;
use winnow::token::{any, take_until, take_while};
use winnow::{PResult, Parser};

use crate::tokens::Include;
use crate::Statement;

static LEFT_GULLWING: char = '{';
static RIGHT_GULLWING: char = '}';

static OPEN_COMMENT: &str = "/*";
static CLOSE_COMMENT: &str = "*/";

pub(crate) fn trim<'a, F, O, E>(inner: F) -> impl Parser<&'a str, O, E>
where
    E: ParserError<&'a str>,
    F: Parser<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

pub(crate) fn between_gullwings<'a>(input: &mut &'a str) -> PResult<&'a str> {
    delimited(
        LEFT_GULLWING,
        take_until(0.., RIGHT_GULLWING),
        RIGHT_GULLWING,
    )
    .parse_next(input)
}

pub(crate) fn not_single_digit<'a>(input: &mut &'a str) -> PResult<&'a str> {
    if input.len() == 1 {
        alpha1.parse_next(input)
    } else {
        Ok(input)
    }
}

pub(crate) fn parse_include_path<'a>(input: &mut &'a str) -> PResult<&'a str> {
    take_while(1.., |c: char| !c.is_whitespace() && c != RIGHT_GULLWING).parse_next(input)
}

// This fails on leading whitespace, may need to fix
// Grabs comments inside the include as arguments, needs fix
pub(crate) fn parse_include_args<'a>(input: &mut &'a str) -> PResult<Vec<&'a str>> {
    let args = separated(
        0..,
        take_while(0.., |c: char| !c.is_whitespace()),
        multispace1,
    )
    .parse_next(input)?;

    Ok(args)
}

pub(crate) fn parse_include<'a>(input: &mut &'a str) -> PResult<Statement<'a>> {
    let mut between = between_gullwings(input)?;
    ("Between: {}", between);

    // Hacky way of keeping ABL params from being parsed as includes
    // Need to revisit this later
    if between.len() == 1 && between.chars().next().unwrap().is_digit(10) {
        return Ok(Statement::Comment(between));
    }

    let (path, args) = seq!(parse_include_path, parse_include_args,).parse_next(&mut between)?;

    Ok(Statement::Include(Include {
        path,
        content: None,
        arguments: args,
    }))
}

pub(crate) fn skip_to_comment_or_include<'a>(input: &mut &'a str) -> PResult<Statement<'a>> {
    let (_skipped, statement): (Vec<char>, Statement<'a>) =
        repeat_till(0.., any, alt((parse_comment, parse_include))).parse_next(input)?;

    return Ok(statement);
}

pub(crate) fn parse_comment<'a>(input: &mut &'a str) -> PResult<Statement<'a>> {
    let comment =
        delimited(OPEN_COMMENT, take_until(0.., CLOSE_COMMENT), CLOSE_COMMENT).parse_next(input)?;

    let count = comment.matches(OPEN_COMMENT).count();
    for _ in 0..count {
        take_until(0.., CLOSE_COMMENT).parse_next(input)?;
    }

    Ok(Statement::Comment(comment))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_between_gullwings() {
        assert_eq!(between_gullwings(&mut "{ include.i }"), Ok(" include.i "));
        assert_eq!(between_gullwings(&mut "{include.i}"), Ok("include.i"));
        assert_eq!(
            between_gullwings(&mut "{ include.i &val1 &val2 &val3 }"),
            Ok(" include.i &val1 &val2 &val3 ")
        );
        assert_eq!(
            between_gullwings(&mut "{include.i &val1 &val2 &val3}"),
            Ok("include.i &val1 &val2 &val3")
        );
    }

    #[test]
    fn test_parse_include_path() {
        assert_eq!(parse_include_path(&mut "include.i a"), Ok("include.i"));
        assert_eq!(parse_include_path(&mut " include.i a"), Ok("include.i"));
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
        assert_eq!(parse_include_args(&mut "1 2 3"), Ok(vec!["1", "2", "3"]));
    }

    #[test]
    fn test_parse_include() {
        assert_eq!(
            parse_include(&mut "{ include.i }"),
            Ok(Statement::Include(Include {
                path: "include.i",
                content: None,
                arguments: vec![""],
            }))
        );
        assert_eq!(
            parse_include(&mut "{ include.i &val1 &val2 &val3 }"),
            Ok(Statement::Include(Include {
                path: "include.i",
                content: None,
                arguments: vec!["&val1", "&val2", "&val3", ""],
            }))
        );
    }
}
