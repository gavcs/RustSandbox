// use std::io;
// use std::cmp::Ordering;
// use rand::Rng;

// fn main() {
//     println!("Guess the number!");
    
//     let mut guess = String::new();
    
//     let target = rand::thread_rng().gen_range(1..=100);
//     println!("The correct number is {target}.");
//     loop {
//         print!("Please input your guess: ");
//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to read line");

//         let guess: u32 = guess.trim().parse().expect("Guess is not a number.");
        
//         println!("You guessed: {guess}");
//         match guess.cmp(&target) {
//             Ordering::Less => println!("Too Low"),
//             Ordering::Greater => println!("Too High"),
//             Ordering::Equal => println!("Correct!"),
//         }
//         let mut guess: String = String::new();
//     }
// }
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        print!("Please input your guess: ");

        io::Write::flush(&mut io::stdout()).expect("Unable to flush.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}