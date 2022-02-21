use std::error;
use std::fmt;

pub type BoxError = std::boxed::Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>;

#[derive(Debug)] //, Clone, PartialEq, Eq)]
pub struct TLEParseError {
    source: Option<BoxError>,
    kind: TLEParseErrorKind,
}

impl error::Error for TLEParseError {}

impl fmt::Display for TLEParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TLEParseError")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TLEParseErrorKind {
    EncodingError,
    InvalidFormat,
}

// impl<E: error::Error + 'static> From<E> for TLEParseError {
//     fn from(e: E) -> Self {
//         Self {
//             source: Box::new(e),
//         }
//     }
// }

impl From<std::str::Utf8Error> for TLEParseError {
    fn from(e: std::str::Utf8Error) -> Self {
        Self {
            source: Some(Box::new(e)),
            kind: TLEParseErrorKind::EncodingError,
        }
    }
}

macro_rules! enforce_line_length_and_termination {
    ($line:ident) => {
        if !($line.len() == 69 || ($line.len() == 70 && $line[79] == b'\n')) {
            return Err(TLEParseError {
                source: None,
                kind: TLEParseErrorKind::InvalidFormat,
            });
        }
        let $line = &$line[..69];
    };
}

macro_rules! get_next_or_incomplete_error {
    ($var:ident, $iter:expr) => {
        let $var = $iter.next().ok_or(TLEParseError {
            source: None,
            kind: TLEParseErrorKind::InvalidFormat,
        })?;
    };
}

macro_rules! invalid_error {
    () => {
        return Err(TLEParseError {
            source: None,
            kind: TLEParseErrorKind::InvalidFormat,
        })
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

fn check_line_length_and_termination(line: &[u8]) -> Result<&[u8], TLEParseError> {
    if !(line.len() == 69 || (line.len() == 70 && line[79] == b'\n')) {
        Err(TLEParseError {
            source: None,
            kind: TLEParseErrorKind::InvalidFormat,
        })
    } else {
        Ok(&line[..69])
    }
}

/**
Structure representing a single TLE.

A two-line element set (TLE) is a data format encoding a list of orbital
elements of an Earth-orbiting object at a given point in time, the epoch.

All the attributes parsed from the TLE are expressed in the same units that
are used in the TLE format.
*/
#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Default)]
pub struct TLE {
    /// Name of the object.
    pub name: String,
    /// Norad number.
    pub norad: String,
    pub classification: char,
    /// International designation.
    pub int_desig: String,
    pub epoch_year: i32,
    pub epoch_day: f64,
    pub dn_o2: f64,
    pub ddn_06: f64,
    pub bstar: f64,
    pub set_num: u32,
    /// Inclination (degrees).
    pub inc: f64,
    /// Right ascension of the ascending node
    /// ([Wikipedia](https://en.wikipedia.org/wiki/Longitude_of_the_ascending_node)).
    pub raan: f64,
    /// Eccentricity (`0 ≤ ecc < 1`).
    pub ecc: f64,
    /// Argument of perigee (degrees).
    pub argp: f64,
    pub M: f64,
    pub n: f64,
    pub rev_num: i32,
}

/// Parse a TLE from a string representing three lines.
pub fn parse(tle_str: &str) -> Result<TLE, TLEParseError> {
    let mut lines_iter = tle_str.lines();
    get_next_or_incomplete_error!(name, lines_iter);
    get_next_or_incomplete_error!(line1, lines_iter);
    get_next_or_incomplete_error!(line2, lines_iter);
    from_lines(name, line1, line2)
}

/// Parse a TLE from the three individual lines.
pub fn from_lines(name: &str, line1: &str, line2: &str) -> Result<TLE, TLEParseError> {
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
