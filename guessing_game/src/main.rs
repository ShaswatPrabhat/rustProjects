use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 100);
    println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess number");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You guessed {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number, try again");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!(" Too Low!"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("Equal");
                break;
            }
        }
    }
}
