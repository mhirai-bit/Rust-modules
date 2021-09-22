use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::fs::File;


fn main() -> io::Result<()> {
    buf_reader()?;
    buf_writer()?;
    buf_reader()?;


    Ok(())
}

fn buf_reader() -> io::Result<()> {
    let f = match File::open("foo.txt") {
        Ok(f) => f,
        Err(_) => {       
            let mut f = File::create("foo.txt")?;
            match f.write(b"Here is the example sentence\n It has two lines") {
                Err(_) => println!("failed to create a file"),
                _ => println!("file has been created"),
            }
            f
        }
    };
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();
    
    // read a line into buffer
    reader.read_line(&mut buffer)?;
    
    println!("{}", buffer);
    Ok(())
    
}

fn buf_writer() -> io::Result<()> {
    let f = File::create("foo.txt")?;
    {
        let mut writer = BufWriter::new(f);

        // write a byte to the buffer
        writer.write(&[42])?;
    } // the buffer is floshed once writer goes out of scope

    Ok(())
}