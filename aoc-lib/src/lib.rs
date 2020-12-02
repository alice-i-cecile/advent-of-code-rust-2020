use std::io::{BufRead, BufReader};
use std::{error::Error, fs::File, path::Path};

pub fn read_integers(path: &Path) -> Result<Vec<isize>, Box<dyn Error>> {
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);

    let int_vec = buf_reader
        .lines()
        .map(|l| -> Result<_, Box<dyn Error>> { Ok(l?.parse()?) })
        .collect::<Result<_, _>>()?;

    Ok(int_vec)
}
