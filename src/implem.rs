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
}

// macro_rules! enforce_line_length_and_termination {
//     ($line:ident) => {
//         if !($line.len() == 69 || ($line.len() == 70 && $line[70] == b'\n')) {
//             return Err(TLEParseError {
//                 source: None,
//                 kind: TLEParseErrorKind::InvalidFormat,
//             });
//         }
//         let $line = &$line[..69];
//     };
// }

macro_rules! get_next_or_incomplete_error {
    ($var:ident, $iter:expr) => {
        let $var = $iter.next().ok_or(Error::InvalidFormat("incomplete"))?;
    };
}

macro_rules! invalid_error {
    () => {
        return Err(Error::InvalidFormat("invalid character"))
    };
}

macro_rules! assert_is {
    ($x:expr, $y:expr) => {
        if $x != $y {
            invalid_error!()
        }
    };
}

macro_rules! assert_is_space {
    ($x:expr) => {
        if $x != b' ' {
            invalid_error!()
        }
    };
}

fn check_line_length_and_termination(line: &[u8]) -> Result<&[u8], Error> {
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
    // enforce_line_length_and_termination!(line1);
    // enforce_line_length_and_termination!(line2);

    let line1 = check_line_length_and_termination(line1)?;
    let line2 = check_line_length_and_termination(line2)?;

    assert_is!(line1[0], b'1');
    assert_is_space!(line1[1]);

    let norad = std::str::from_utf8(&line1[2..7])?.to_string();
    let classification = line1[7] as char;

    assert_is!(line2[0], b'2');
    assert_is_space!(line2[1]);

    let norad2 = std::str::from_utf8(&line2[2..7])?.to_string();
    if norad2 != norad {
        invalid_error!()
    }

    Ok(TLE {
        name,
        norad,
        classification,
        ..TLE::default()
    })
}
