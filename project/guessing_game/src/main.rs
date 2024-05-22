use std::io;

fn main() {
    println!("Guess the number!");
    print!("Please input your guess: ");
    let mut guess = String::new();
    let found = false;
    while !found {
        io::Write::flush(&mut io::stdout()).expect("Write failed");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: usize = guess.trim().parse().expect("Guess is not a number.");
        
        print!("You guessed: {guess}");
    }
}