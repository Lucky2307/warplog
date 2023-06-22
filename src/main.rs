mod extract_link;

use std::fs::File;
use std::io::Read;
use std::str::from_utf8_unchecked;

use crate::extract_link::extract_link::from_string;
fn main() -> std::io::Result<()> {
    let mut data = File::open("data_3")?;
    let mut buffer = vec![];
    data.read_to_end(&mut buffer)?;

    let data_string = unsafe { from_utf8_unchecked(&buffer) };

    let link_struct = from_string(data_string);

    println!("{:#?}", link_struct);
    Ok(())
}
