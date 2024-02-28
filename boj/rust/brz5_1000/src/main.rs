use std::io;

fn main() -> io::Result<()> {
    let mut input = String::from("");

    io::stdin().read_line(&mut input)?;

    input = input.trim().to_string();

    let a: u8 = input.split(" ").collect::<Vec<&str>>()[0].parse().unwrap();
    let b: u8 = input.split(" ").collect::<Vec<&str>>()[1].parse().unwrap();

    println!("{}", a + b);

    Ok(())
}
