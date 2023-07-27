use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::not_line_ending;
use nom::combinator::opt;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

pub fn override_path(source: &str) -> IResult<&str, Option<String>> {
    let (source, _) = pair(take_until("\n-- attributes"), tag("\n-- attributes"))(source)?;
    let (source, the_path) = opt(preceded(
        pair(take_until("-- path: "), tag("-- path: ")),
        not_line_ending.map(|s: &str| s.to_string()),
    ))(source)?;
    Ok((source, the_path))
}
