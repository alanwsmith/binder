use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::not_line_ending;
use nom::combinator::opt;
use nom::IResult;

pub fn filter_status(source: &str) -> IResult<&str, bool> {
    let (source, check_status_1) = opt(take_until("\n-- attributes"))(source)?;
    match check_status_1 {
        Some(_) => {
            let (source, _) = tag("\n-- attributes")(source)?;
            let (source, check_status_2) = opt(take_until("-- status: "))(source)?;
            match check_status_2 {
                Some(_) => {
                    let (source, _) = tag("-- status: ")(source)?;
                    let (source, b) = not_line_ending(source)?;
                    match b.trim() {
                        "published" => Ok((source, true)),
                        "draft" => Ok((source, true)),
                        "scratch" => Ok((source, true)),
                        _ => Ok((source, false)),
                    }
                }
                None => Ok((source, false)),
            }
        }
        None => Ok((source, false)),
    }
}
