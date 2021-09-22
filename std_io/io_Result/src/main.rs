use std::io;

fn main() -> io::Result<()> {
    read_input()?;
    Ok(())
}

fn read_input() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    println!("You typed: {}", input.trim());

    Ok(())
}