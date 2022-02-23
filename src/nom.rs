use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_while_m_n};
use nom::character::complete::{line_ending, not_line_ending};
use nom::combinator::{all_consuming, opt};
use nom::IResult;

use thiserror::Error;

use crate::TLE;

#[derive(Debug, Error)]
#[error("invalid TLE string")]
pub struct Error;

impl From<nom::Err<nom::error::Error<&str>>> for Error {
    fn from(_: nom::Err<nom::error::Error<&str>>) -> Error {
        Error
    }
}

impl std::str::FromStr for TLE {
    type Err = Error;

    fn from_str(s: &str) -> Result<TLE, Error> {
        let (_, tle) = parse_single_tle(s)?;
        Ok(tle)
    }
}

fn parse_single_tle(s: &str) -> IResult<&str, TLE> {
    let (rest, (line_0, line_1, _line_2)) = all_consuming(segment_lines)(s)?;
    let (_, (norad, int_desig, classification)) = all_consuming(parse_line_1)(line_1)?;
    Ok((
        rest,
        TLE {
            name: line_0.to_string(),
            norad: norad.to_string(),
            int_desig: int_desig.to_string(),
            classification,
            ..TLE::default()
        },
    ))
}

fn segment_lines(s: &str) -> IResult<&str, (&str, &str, &str)> {
    let (s, line_0) = take_while_m_n(1usize, 24usize, |c| c != '\n')(s)?;
    let (s, _) = line_ending(s)?;
    let (s, line_1) = take_while_m_n(69usize, 69usize, |c| c != '\n')(s)?;
    let (s, _) = line_ending(s)?;
    let (s, line_2) = take_while_m_n(69usize, 69usize, |c| c != '\n')(s)?;
    let (s, _) = opt(line_ending)(s)?;
    Ok((s, (line_0, line_1, line_2)))
}

fn parse_line_1(s: &str) -> IResult<&str, (&str, &str, char)> {
    let (s, _) = tag("1 ")(s)?;
    let (s, norad) = take(5usize)(s)?;
    let (s, classification) = alt((tag("C"), tag("U"), tag("S")))(s)?;
    let (s, _) = tag(" ")(s)?;
    let (s, int_desig) = take(8usize)(s)?;
    Ok((
        s,
        (
            norad,
            int_desig,
            classification
                .chars()
                .next()
                .expect("cannot get classification char"),
        ),
    ))
}

// fn tle_name(s: &str) -> IResult<&str, &str> {
//     not_line_ending(s)
// }

#[cfg(test)]
mod tests {
    use super::*;

    use nom::error::{Error, ErrorKind};

    //     const ISS: &str = "ISS (ZARYA)
    // 1 25544U 98067A   20045.18587073  .00000950  00000-0  25302-4 0  9990
    // 2 25544  51.6443 242.0161 0004885 264.6060 207.3845 15.49165514212791
    // ";

    #[test]
    fn test_segment_lines() {
        let line_0 = "ISS (ZARYA)";
        let line_1 = "1 25544U 98067A   20045.18587073  .00000950  00000-0  25302-4 0  9990";
        let line_2 = "2 25544  51.6443 242.0161 0004885 264.6060 207.3845 15.49165514212791";

        let tle_string = "ISS (ZARYA)
1 25544U 98067A   20045.18587073  .00000950  00000-0  25302-4 0  9990
2 25544  51.6443 242.0161 0004885 264.6060 207.3845 15.49165514212791
"; // newline at the end

        assert_eq!(
            segment_lines(tle_string),
            Ok(("", (line_0, line_1, line_2)))
        );

        let tle_string = "ISS (ZARYA)
1 25544U 98067A   20045.18587073  .00000950  00000-0  25302-4 0  9990
2 25544  51.6443 242.0161 0004885 264.6060 207.3845 15.49165514212791";

        assert_eq!(
            segment_lines(tle_string),
            Ok(("", (line_0, line_1, line_2)))
        );

        let tle_string = "ISS (ZARYA)
1 25544U 98067A   20045.18587073  .00000950  00000-0  25302-4 0  9990X
2 25544  51.6443 242.0161 0004885 264.6060 207.3845 15.49165514212791
";

        assert_eq!(
            segment_lines(tle_string),
            Err(nom::Err::Error(Error {
                input: "X
2 25544  51.6443 242.0161 0004885 264.6060 207.3845 15.49165514212791
",
                code: ErrorKind::CrLf
            }))
        );

        let tle_string = "ISS (ZARYA)
1 25544U 98067A   20045.18587073  .00000950  00000-0  25302-4 0  9990
2 25544  51.6443 242.0161 0004885 264.6060 207.3845 15.4916551421279";

        assert_eq!(
            segment_lines(tle_string),
            Err(nom::Err::Error(Error {
                input: "2 25544  51.6443 242.0161 0004885 264.6060 207.3845 15.4916551421279",
                code: ErrorKind::TakeWhileMN
            }))
        );

        let tle_string = "ISS (ZARYA)
1 25544U 98067A   20045.18587073  .00000950  00000-0  25302-4 0  9990
2 25544  51.6443 242.0161 0004885 264.6060\n207.3845 15.49165514212791";

        assert_eq!(
            segment_lines(tle_string),
            Err(nom::Err::Error(Error {
                input: "2 25544  51.6443 242.0161 0004885 264.6060\n207.3845 15.49165514212791",
                code: ErrorKind::TakeWhileMN
            }))
        );
    }

    #[test]
    fn parse_tle_name() {
        let tle_string = "ISS (ZARYA)
1 25544U 98067A   20045.18587073  .00000950  00000-0  25302-4 0  9990
2 25544  51.6443 242.0161 0004885 264.6060 207.3845 15.49165514212791
";
        let rest = "
1 25544U 98067A   20045.18587073  .00000950  00000-0  25302-4 0  9990
2 25544  51.6443 242.0161 0004885 264.6060 207.3845 15.49165514212791
";
        assert_eq!(tle_name(tle_string), Ok((rest, "ISS (ZARYA)")));
    }
}
