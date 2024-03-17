use itertools::Itertools;
use std::fs;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    for date_folder in fs::read_dir("data")? {
        for filename in fs::read_dir(date_folder?.path())? {
            let filename = filename?;
            println!("\n{:?}", filename.path());
            let file = fs::File::open(filename.path())?;
            let buf_reader = BufReader::new(file);
            for mut chunk in &buf_reader.lines().chunks(3) {
                let name = chunk.next().unwrap()?;
                let line1 = chunk.next().unwrap()?;
                let line2 = chunk.next().unwrap()?;
                // let tle_str = [name.as_str(), line1.as_str(), line2.as_str()].join("\n");
                let tle = tletools::from_lines(name.as_str(), line1.as_str(), line2.as_str());
                let sgp = sgp4::Elements::from_tle(Some(name), line1.as_bytes(), line2.as_bytes())
                    .unwrap();
                println!("{:#?}", tle);
                println!("{}", serde_json::to_string(&sgp).unwrap());
            }
        }
    }

    Ok(())
}
