use std::io;

fn main() -> io::Result<()> {
    let mut input = String::from("");

    io::stdin().read_line(&mut input)?;

    let N: u8 = input.trim().parse().unwrap();

    for i in 0..(N - 1) {
        for _ in 0..(i + 1) {
            print!("*");
        }

        for _ in 0..(2 * N - 2 * (i + 1)) {
            print!(" ");
        }

        for _ in 0..(i + 1) {
            print!("*");
        }

        println!("");
    }

    for _ in 0..(2 * N) {
        print!("*");
    }
    println!("");

    for i in (0..(N - 1)).rev() {
        for _ in 0..(i + 1) {
            print!("*");
        }

        for _ in 0..(2 * N - 2 * (i + 1)) {
            print!(" ");
        }

        for _ in 0..(i + 1) {
            print!("*");
        }

        println!("");
    }

    Ok(())
}
