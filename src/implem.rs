use thiserror::Error;

use crate::TLE;

pub type BoxError = std::boxed::Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[error("invalid TLE string")]
pub enum Error {
    #[error("invalid encoding")]
    EncodingError(#[from] std::str::Utf8Error),
    #[error("invalid format, reason: {0}")]
    InvalidFormat(&'static str),
    #[error("cannot parse {0}")]
    ParseError(&'static str),
    #[error("cannot parse int")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("cannot parse float")]
    ParseFloatError(#[from] std::num::ParseFloatError),
}

macro_rules! get_next_or_incomplete_error {
    ($var:ident, $iter:expr) => {
        let $var = $iter.next().ok_or(Error::InvalidFormat("incomplete"))?;
    };
}

macro_rules! invalid_error {
    ($msg:expr) => {
        return Err(Error::InvalidFormat($msg))
    };
}

macro_rules! ensure_is {
    ($x:expr, $y:expr) => {
        if $x != $y {
            invalid_error!("wrong character")
        }
    };
}

macro_rules! ensure_is_space {
    ($x:expr) => {
        if $x != b' ' {
            invalid_error!("expected space character")
        }
    };
}

fn ensure_line_length_and_termination(line: &[u8]) -> Result<&[u8], Error> {
    if !(line.len() == 69 || (line.len() == 70 && line[69] == b'\n')) {
        Err(Error::InvalidFormat("incorrect line length"))
    } else {
        Ok(&line[..69])
    }
}

/// Parse a TLE from a string representing three lines.
pub fn parse(tle_str: &str) -> Result<TLE, Error> {
    let mut lines_iter = tle_str.lines();
    get_next_or_incomplete_error!(name, lines_iter);
    get_next_or_incomplete_error!(line1, lines_iter);
    get_next_or_incomplete_error!(line2, lines_iter);
    from_lines(name, line1, line2)
}

/// Parse a TLE from the three individual lines.
pub fn from_lines(name: &str, line1: &str, line2: &str) -> Result<TLE, Error> {
    let name = name.trim().to_string();
    let line1 = line1.as_bytes();
    let line2 = line2.as_bytes();

    // LINE 1

    let line1 = ensure_line_length_and_termination(line1)?;

    ensure_is!(line1[0], b'1');
    ensure_is_space!(line1[1]);

    let norad = std::str::from_utf8(&line1[2..=6])?.to_string();
    let classification = line1[7] as char;

    ensure_is_space!(line1[8]);

    let int_desig = std::str::from_utf8(&line1[9..=16])?.trim().to_string();

    ensure_is_space!(line1[17]);

    let y = std::str::from_utf8(&line1[18..=19])?.parse::<u8>()?;
    let epoch_year = if y <= 56 {
        2000 + y as i32
    } else {
        1900 + y as i32
    };

    let epoch_day = std::str::from_utf8(&line1[20..=31])?.parse::<f64>()?;

    ensure_is_space!(line1[32]);

    let dn_o2 = std::str::from_utf8(&line1[33..=42])?
        .trim()
        .parse::<f64>()?;

    ensure_is_space!(line1[43]);

    // ddn_u6

    ensure_is_space!(line1[52]);

    // bstar

    ensure_is_space!(line1[61]);
    ensure_is!(line1[62], b'0');
    ensure_is_space!(line1[63]);

    let set_num = std::str::from_utf8(&line1[64..=67])?
        .trim()
        .parse::<u32>()?;

    // LINE 2

    let line2 = ensure_line_length_and_termination(line2)?;

    ensure_is!(line2[0], b'2');
    ensure_is_space!(line2[1]);

    let norad2 = std::str::from_utf8(&line2[2..=6])?.to_string();
    if norad2 != norad {
        invalid_error!("norad on line 1 and 2 are different");
    }

    ensure_is_space!(line2[7]);

    let inc = std::str::from_utf8(&line2[8..=15])?.trim().parse::<f64>()?;

    ensure_is_space!(line2[16]);

    let raan = std::str::from_utf8(&line2[17..=24])?
        .trim()
        .parse::<f64>()?;

    ensure_is_space!(line2[25]);

    let e = std::str::from_utf8(&line2[26..=32])?.parse::<u32>()?;
    let ecc = e as f64 / 10_000_000f64;

    ensure_is_space!(line2[33]);

    let argp = std::str::from_utf8(&line2[34..=41])?
        .trim()
        .parse::<f64>()?;

    ensure_is_space!(line2[42]);

    #[allow(non_snake_case)]
    let M = std::str::from_utf8(&line2[43..=50])?
        .trim()
        .parse::<f64>()?;

    ensure_is_space!(line2[51]);

    let n = std::str::from_utf8(&line2[52..=62])?
        .trim()
        .parse::<f64>()?;

    let rev_num = std::str::from_utf8(&line2[63..=67])?
        .trim()
        .parse::<u32>()?;

    Ok(TLE {
        name,
        norad,
        classification,
        int_desig,
        epoch_year,
        epoch_day,
        dn_o2,
        set_num,
        inc,
        raan,
        ecc,
        argp,
        M,
        n,
        rev_num,
        ..TLE::default()
    })
}
