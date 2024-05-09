use std::io;

fn main() {
    let mut x = String::new();

    io::stdin().read_line(&mut x).expect("Failed to read line");

    let x: u32 = x.trim().parse().expect("Failed to read integer");

    let result = sqrt(x);
    println!("{x} squared equals {result}");
}


fn sqrt(x: u32) -> u32{
    x*x
}