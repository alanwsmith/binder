use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::not_line_ending;
use nom::combinator::opt;
use nom::IResult;

pub fn filter_site<'a>(source: &'a str, site_id: &'a str) -> IResult<&'a str, bool> {
    let (source, check_site_1) = opt(take_until("\n-- attributes"))(source)?;
    match check_site_1 {
        Some(_) => {
            let (source, _) = tag("\n-- attributes")(source)?;
            let (source, check_site_2) = opt(take_until("-- site: "))(source)?;
            match check_site_2 {
                Some(_) => {
                    let (source, _) = tag("-- site: ")(source)?;
                    let (source, the_id) = not_line_ending(source)?;
                    if the_id.trim() == site_id {
                        Ok((source, true))
                    } else {
                        Ok((source, false))
                    }
                }
                None => Ok((source, false)),
            }
        }
        None => Ok((source, false)),
    }
}
