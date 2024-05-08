use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The element at index {index} is {element}");
}