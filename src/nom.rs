use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_while_m_n};
use nom::character::complete::{line_ending, not_line_ending};
use nom::character::is_digit;
use nom::combinator::{all_consuming, opt};
use nom::sequence::separated_pair;
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

fn tle_name(s: &str) -> IResult<&str, &str> {
    not_line_ending(s)
}

fn f64_3_4_digits(s: &[u8]) -> IResult<&[u8], f64> {
    let (s, (a, b)) = separated_pair(u16_3_digits, tag("."), u16_4_digits)(s)?;
    let x = a as f64 + b as f64 / 10000.;
    Ok((s, x))
}

fn u8_2_digits(s: &[u8]) -> IResult<&[u8], u8> {
    let (s, digits) = take_while_m_n(2usize, 2usize, is_digit)(s)?;
    let n = (digits[0] - b'0') as u8 * 10 + (digits[1] - b'0') as u8;
    Ok((s, n))
}

fn u16_3_digits(s: &[u8]) -> IResult<&[u8], u16> {
    let (s, digits) = take_while_m_n(3usize, 3usize, is_digit)(s)?;
    let n = (digits[0] - b'0') as u16 * 100
        + (digits[1] - b'0') as u16 * 10
        + (digits[2] - b'0') as u16;
    Ok((s, n))
}

fn u16_4_digits(s: &[u8]) -> IResult<&[u8], u16> {
    let (s, digits) = take_while_m_n(4usize, 4usize, is_digit)(s)?;
    let n = (digits[0] - b'0') as u16 * 1000
        + (digits[1] - b'0') as u16 * 100
        + (digits[2] - b'0') as u16 * 10
        + (digits[3] - b'0') as u16;
    Ok((s, n))
}

fn u32_5_digits(s: &[u8]) -> IResult<&[u8], u32> {
    let (s, digits) = take_while_m_n(5usize, 5usize, is_digit)(s)?;
    let n = (digits[0] - b'0') as u32 * 10000
        + (digits[1] - b'0') as u32 * 1000
        + (digits[2] - b'0') as u32 * 100
        + (digits[3] - b'0') as u32 * 10
        + (digits[4] - b'0') as u32;
    Ok((s, n))
}

fn u32_7_digits(s: &[u8]) -> IResult<&[u8], u32> {
    let (s, digits) = take_while_m_n(7usize, 7usize, is_digit)(s)?;
    let n = (digits[0] - b'0') as u32 * 1000000
        + (digits[1] - b'0') as u32 * 100000
        + (digits[2] - b'0') as u32 * 10000
        + (digits[3] - b'0') as u32 * 1000
        + (digits[4] - b'0') as u32 * 100
        + (digits[5] - b'0') as u32 * 10
        + (digits[6] - b'0') as u32;
    Ok((s, n))
}

fn u32_8_digits(s: &[u8]) -> IResult<&[u8], u32> {
    let (s, digits) = take_while_m_n(8usize, 8usize, is_digit)(s)?;
    let n = (digits[0] - b'0') as u32 * 10000000
        + (digits[1] - b'0') as u32 * 1000000
        + (digits[2] - b'0') as u32 * 100000
        + (digits[3] - b'0') as u32 * 10000
        + (digits[4] - b'0') as u32 * 1000
        + (digits[5] - b'0') as u32 * 100
        + (digits[6] - b'0') as u32 * 10
        + (digits[7] - b'0') as u32;
    Ok((s, n))
}

#[cfg(test)]
mod tests {
    use super::*;

    use nom::error::{Error, ErrorKind};

    //     const ISS: &str = "ISS (ZARYA)
    // 1 25544U 98067A   20045.18587073  .00000950  00000-0  25302-4 0  9990
    // 2 25544  51.6443 242.0161 0004885 264.6060 207.3845 15.49165514212791
    // ";

    #[test]
    fn test_f64_3_4_digits() {
        const N: i32 = 10000;
        for n in 0..N {
            let x = n as f64 / N as f64;
            let string = format!("{:08.4}", x);
            let s = string.as_bytes();
            match f64_3_4_digits(s) {
                Ok((s, y)) => {
                    assert_eq!(s, b"");
                    assert_eq!(y, x);
                }
                e => {
                    panic!("parser returned {e:?}");
                }
            }
        }
    }

    #[test]
    fn test_u8_2_digits() {
        for n in 0..=99 {
            let string = format!("{:02}", n);
            let s = string.as_bytes();
            match u8_2_digits(s) {
                Ok((s, m)) => {
                    assert_eq!(s, b"");
                    assert_eq!(m, n);
                }
                e => {
                    panic!("parser returned {e:?}");
                }
            }
        }
    }

    #[test]
    fn test_u16_3_digits() {
        for n in 0..=999 {
            let string = format!("{:03}", n);
            let s = string.as_bytes();
            match u16_3_digits(s) {
                Ok((s, m)) => {
                    assert_eq!(s, b"");
                    assert_eq!(m, n);
                }
                e => {
                    panic!("parser returned {e:?}");
                }
            }
        }
    }

    #[test]
    fn test_u16_4_digits() {
        for n in 0..=9999 {
            let string = format!("{:04}", n);
            let s = string.as_bytes();
            match u16_4_digits(s) {
                Ok((s, m)) => {
                    assert_eq!(s, b"");
                    assert_eq!(m, n);
                }
                e => {
                    panic!("parser returned {e:?}");
                }
            }
        }
    }

    #[test]
    fn test_u32_5_digits() {
        for n in 0..=99999 {
            let string = format!("{:05}", n);
            let s = string.as_bytes();
            match u32_5_digits(s) {
                Ok((s, m)) => {
                    assert_eq!(s, b"");
                    assert_eq!(m, n);
                }
                e => {
                    panic!("parser returned {e:?}");
                }
            }
        }
    }

    #[test]
    #[ignore]
    fn test_u32_7_digits() {
        for n in (0..=9999999).skip(7) {
            let string = format!("{:07}", n);
            let s = string.as_bytes();
            match u32_7_digits(s) {
                Ok((s, m)) => {
                    assert_eq!(s, b"");
                    assert_eq!(m, n);
                }
                e => {
                    panic!("parser returned {e:?}");
                }
            }
        }
    }

    #[test]
    #[ignore]
    fn test_u32_8_digits() {
        for n in (0..=99999999).skip(71) {
            let string = format!("{:08}", n);
            let s = string.as_bytes();
            match u32_8_digits(s) {
                Ok((s, m)) => {
                    assert_eq!(s, b"");
                    assert_eq!(m, n);
                }
                e => {
                    panic!("parser returned {e:?}");
                }
            }
        }
    }

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
