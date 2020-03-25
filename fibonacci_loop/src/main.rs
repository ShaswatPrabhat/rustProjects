use std::io;

fn main() {
    println!("Please enter the number of terms: ");
    let mut number_of_terms = String::new();
    io::stdin().read_line(&mut number_of_terms).expect("Failed to read line");

    let number_of_terms: u32 = match number_of_terms.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Please enter a valid number");
            0
        }
    };

    let mut term1: i128 = -1;
    let mut term2: i128 = 1;
    let mut counter = 0;

    while counter <= number_of_terms {
        let term3: i128 = term1 + term2;
        term1 = term2;
        term2 = term3;

        println!("Element number {} is {}", counter, term3);

        counter += 1;
    }
}
