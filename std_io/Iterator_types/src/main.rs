use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
    let mut f = File::create("foo.txt")?;
    f.write(b"This is a sentence\nThis is the second line\nThis is the third line")?;
    let f = File::open("foo.txt")?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}