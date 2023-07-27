use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::is_not;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::sequence::terminated;
use nom::IResult;

pub fn output_dir_name<'a>(source: &'a str, id: &'a str) -> IResult<&'a str, String> {
    let (source, _) = multispace0(source.trim())?;
    let (source, parts) =
        many_till(terminated(is_not(" -.'"), alt((is_a(" -.'"), eof))), eof)(source)?;
    let response = format!(
        "{}--{}",
        parts
            .0
            .iter()
            .map(|p| p.to_lowercase())
            .collect::<Vec<String>>()
            .join("-"),
        id
    );
    Ok((source, response))
}
