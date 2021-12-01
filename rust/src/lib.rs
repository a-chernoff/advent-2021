use std::io::{BufReader, Result};

use std::fs::File;
use std::path::Path;

pub fn input_data_bufreader<P>(input_num: P) -> Result<BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(Path::new("../input").join(input_num))?;
    Ok(BufReader::new(file))
}
