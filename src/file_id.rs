use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::IResult;

pub fn file_id(source: &str) -> IResult<&str, &str> {
    let (a, _b) = take_until("\n-- attributes")(source)?;
    let (a, _b) = tag("\n-- attributes")(a)?;
    let (a, _b) = take_until("-- id: ")(a)?;
    let (a, _b) = tag("-- id: ")(a)?;
    let (a, _b) = multispace0(a)?;
    let (_a, b) = not_line_ending(a)?;
    Ok(("", b.trim()))
}
