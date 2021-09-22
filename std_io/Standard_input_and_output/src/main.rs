use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    std_input();
    std_output();

    Ok(())
}

fn std_input() -> io::Result<()> {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input)?;
    
    println!("You typed: {}", input.trim());
    Ok(())
}

fn std_output() -> io::Result<()> {
    io::stdout().write(&[42])?;
    Ok(())
}