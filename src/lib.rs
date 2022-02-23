#![cfg_attr(doc, feature(doc_auto_cfg))]

//! **TLE-tools** is a small library to work with [`two-line element set`] files.
//!
//! This module defines the structure [`TLE`] which represents a single TLE.
//! Its fields are expressed in the same units that are used by the TLE format.
//!
//! [`two-line element set`]: https://en.wikipedia.org/wiki/Two-line_element_set

/// Structure representing a single TLE.
///
/// A two-line element set (TLE) is a data format encoding a list of orbital
/// elements of an Earth-orbiting object at a given point in time, the epoch.
///
/// All the fields parsed from the TLE are expressed in the same units
/// that are used in the TLE format.
#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Default)]
pub struct TLE {
    /// Name of the object.
    pub name: String,
    /// Norad number.
    pub norad: String,
    /// Classification (`U`: unclassified, `C`: classified, `S`: secret)
    pub classification: char,
    /// International designator (year, launch number, piece).
    pub int_desig: String,
    /// Year of the epoch.
    pub epoch_year: i32,
    /// Day of the year plus fraction of the day.
    pub epoch_day: f64,
    /// First time-derivative of the mean motion divided by 2.
    pub dn_o2: f64,
    /// Second time-derivative of the mean motion divided by 6.
    pub ddn_o6: f64,
    /// BSTAR coefficient ([Wikipedia](https://en.wikipedia.org/wiki/BSTAR)).
    pub bstar: f64,
    /// Element set number.
    pub set_num: u32,
    /// Inclination.
    pub inc: f64,
    /// Right ascension of the ascending node
    /// ([Wikipedia](https://en.wikipedia.org/wiki/Longitude_of_the_ascending_node)).
    pub raan: f64,
    /// Eccentricity (`0 ≤ ecc < 1`).
    pub ecc: f64,
    /// Argument of perigee.
    pub argp: f64,
    /// Mean anomaly.
    pub M: f64,
    /// Mean motion.
    pub n: f64,
    /// Revolution number.
    pub rev_num: u32,
}

mod implem;
pub use implem::*;

#[cfg(feature = "nom")]
pub mod nom;
