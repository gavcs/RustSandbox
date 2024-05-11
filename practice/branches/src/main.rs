use std::io;

fn main() {
    print!("Enter a number less than 34: ");
    let mut num = String::new();

    io::Write::flush(&mut io::stdout()).expect("Write failed");
    
    io::stdin().read_line(&mut num).expect("Failed to read line");

    let num: i32 = num.trim().parse().expect("Failed to create integer");

    if num < 34 {
        println!("The number entered is less than 34");
    } else if num > 34 {
        println!("The number entered is less than 34");
    } else {
        println!("The number entered is 34");
    }
}