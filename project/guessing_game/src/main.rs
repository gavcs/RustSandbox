use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    print!("Please input your guess: ");
    let mut guess = String::new();
    
    let target = rand::thread_rng().gen_range(1..=100);
    io::Write::flush(&mut io::stdout()).expect("Write failed");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: usize = guess.trim().parse().expect("Guess is not a number.");
    println!("The correct number is {target}.");
    println!("You guessed: {guess}");
    match guess.cmp(&target) {
        Ordering::Less => println!("Too Low"),
        Ordering::Greater => println!("Too High"),
        Ordering::Equal => println!("Correct!"),
    }
}