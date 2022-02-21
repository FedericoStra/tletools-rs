// This is a modified version of
// https://github.com/pbrandt1/rust-tle/blob/master/src/lib.rs
// which does not unnecessarily print
// https://github.com/pbrandt1/rust-tle/blob/master/src/lib.rs#L113

use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct TLE {
    pub name: String,
    pub satellite_number: u32,
    pub classification: char,
    pub international_designator: String,
    pub date: String,
    pub first_derivative_mean_motion: f64,
    pub second_derivative_mean_motion: f64,
    pub bstar: f64,
    pub element_set_number: u32,
    pub inclination: f64,
    pub right_ascension: f64,
    pub eccentricity: f64,
    pub argument_of_perigee: f64,
    pub mean_anomaly: f64,
    pub mean_motion: f64,
    pub revolution_number: u32,
}

impl Display for TLE {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        //
        //  Line 1 stuff
        //
        let first_derivative_mean_motion = self.first_derivative_mean_motion / 2.0;
        let first_derivative_mean_motion_str = format!("{:+10.8}", first_derivative_mean_motion);
        let first_derivative_mean_motion_str =
            str::replace(&first_derivative_mean_motion_str, "0.", ".");

        // the value -12345-6 corresponds to -0.12345 × 10-6
        let bstar = str::replace(&format!("{:.4e}", self.bstar * 10.0), "e-", "-"); // times 10 b/c we remove the decimal point later
        let bstar = str::replace(&bstar, "e0", "-0"); // make sure it's 00000-0 not 00000+0
        let bstar = str::replace(&bstar, "e", "+");
        let bstar = str::replace(&bstar, ".", "");

        let second_derivative_mean_motion = str::replace(
            &format!("{:.4e}", self.second_derivative_mean_motion * 10.0),
            "e-",
            "-",
        );
        let second_derivative_mean_motion =
            str::replace(&second_derivative_mean_motion, "e0", "-0");
        let second_derivative_mean_motion = str::replace(&second_derivative_mean_motion, "e", "+");
        let second_derivative_mean_motion = str::replace(&second_derivative_mean_motion, ".", "");

        let line1 = format!(
            "1 {:5}{:1} {:8} {:14} {:10} {:>8} {:>8} 0 {:4}",
            self.satellite_number,
            self.classification,
            self.international_designator,
            self.date,
            first_derivative_mean_motion_str,
            second_derivative_mean_motion,
            bstar,
            self.element_set_number
        );

        let mut checksum1 = 0;

        for c in line1.chars() {
            if c.is_digit(10) {
                checksum1 = checksum1 + c.to_digit(10).unwrap();
            } else if c == '-' {
                checksum1 = checksum1 + 1;
            }
        }

        checksum1 = checksum1 % 10;

        //
        // Line 2 stuff
        //
        let line2 = format!(
            "2 {:5} {:8} {:8} {:7} {:8} {:8} {:11}{:5}",
            self.satellite_number,
            self.inclination,
            self.right_ascension,
            str::replace(&format!("{}", self.eccentricity), "0.", ""),
            self.argument_of_perigee,
            self.mean_anomaly,
            self.mean_motion,
            self.revolution_number
        );

        let mut checksum2 = 0;

        for c in line1.chars() {
            if c.is_digit(10) {
                checksum2 = checksum2 + c.to_digit(10).unwrap();
            } else if c == '-' {
                checksum2 = checksum2 + 1;
            }
        }

        checksum2 = checksum2 % 10;

        write!(
            f,
            "{}\n{}{}\n{}{}",
            self.name, line1, checksum1, line2, checksum2
        )
    }
}

/*
ISS (ZARYA)
1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927
2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537
 */

// "ISS (ZARYA)\n1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927\n2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537"
pub fn parse(tle_str: &str) -> TLE {
    // let name = match &tle_str[0..23].lines().next() {
    //     None => String::From("Unidentified"),
    //     Some(s) => String::From(s)
    // };
    let lines: Vec<&str> = tle_str.lines().collect();
    // println!("{:?}", lines);

    // CHECK NUMBER OF lines
    // VERIFY THE CHECKSUMS
    // check that the two satellite_number values are the same

    // Name line, if exists
    let name = if lines.len() == 3 {
        String::from(lines[0])
    } else {
        String::from("")
    };

    // TLE line 1
    let line1 = if lines.len() == 3 { lines[1] } else { lines[0] };

    // TLE line 2
    let line2 = if lines.len() == 3 { lines[2] } else { lines[3] };

    /*
    ISS (ZARYA)
    1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927
    2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537
     */
    // line 1
    let satellite_number = line1[2..7]
        .trim()
        .parse()
        .expect("Could not parse field 'number'");
    let classification: char = line1.chars().nth(7).unwrap();
    let international_designator = String::from((&line1[9..17]).trim());
    // meh cannot figure out UTC timestamp right now
    // let mut year: u32 = line1[18..20].trim().parse().expect("Could not parse epoch year");
    // if year < 57 {
    //     year = 2000 + year;
    // } else {
    //     year = 1900 + year;
    // }
    let date = format!("{}", &line1[18..32]);
    let first_derivative_mean_motion = (line1[33..43]
        .trim()
        .parse::<f64>()
        .expect("Could not parse field 'first derivative mean motion'"))
        * 2.0;
    let second_derivative_mean_motion_mantissa: f64 =
        format!("{}0.{}", &line1[44..45], &line1[45..50])
            .trim()
            .parse::<f64>()
            .expect("Could not parse field 'second derivative mean motion' mantissa");
    let second_derivative_mean_motion_exponent: f64 = line1[50..53]
        .trim()
        .parse::<f64>()
        .expect("Could not parse field 'second derivative mean motion' exponent");
    let second_derivative_mean_motion = second_derivative_mean_motion_mantissa
        * 10f64.powf(second_derivative_mean_motion_exponent)
        / 6.0;
    let bstar_mantissa: f64 = format!("{}0.{}", &line1[53..54], &line1[54..59])
        .trim()
        .parse::<f64>()
        .expect("Could not parse field 'second derivative mean motion' mantissa");
    let bstar_exponent: f64 = line1[59..61]
        .trim()
        .parse::<f64>()
        .expect("Could not parse field 'second derivative mean motion' exponent");
    let bstar = bstar_mantissa * 10f64.powf(bstar_exponent);
    let element_set_number = line1[64..68]
        .trim()
        .parse()
        .expect("Could not parse field 'element set number'");

    // line 2
    let inclination = line2[8..16]
        .trim()
        .parse()
        .expect("Could not parse field 'inclination'");
    let right_ascension = line2[17..25]
        .trim()
        .parse()
        .expect("Could not parse field 'right ascension'");
    let eccentricity = format!("0.{}", &line2[26..33])
        .trim()
        .parse()
        .expect("Could not parse field 'eccentricity'");
    let argument_of_perigee = line2[34..42]
        .trim()
        .parse()
        .expect("Could not parse field 'argument of perigee'");
    let mean_anomaly = line2[43..51]
        .trim()
        .parse()
        .expect("Could not parse field 'mean anomaly'");
    let mean_motion = line2[52..63]
        .trim()
        .parse()
        .expect("Could not parse field 'mean motion");
    let revolution_number = line2[63..68]
        .trim()
        .parse()
        .expect("Could not parse field 'revolution number'");

    TLE {
        name,
        satellite_number,
        classification,
        international_designator,
        date,
        first_derivative_mean_motion,
        second_derivative_mean_motion,
        bstar,
        element_set_number,
        inclination,
        right_ascension,
        eccentricity,
        argument_of_perigee,
        mean_anomaly,
        mean_motion,
        revolution_number,
    }
}

// fun parse_f64(str, name) -> f64 {
//     let value = str
//         .trim()
//         .parse()
//         .expect(format!("Failed parsing {}, tried \"{}\"", name, str))
// }
