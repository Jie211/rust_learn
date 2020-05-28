use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // print!("The secret number is: {}\n", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        // & -> reference 
        // read_line return a Result -> enumrations(enum)

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("bad input __> {}", err);
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => print!("small\n"),
            Ordering::Greater => print!("big\n"),
            Ordering::Equal => {
                print!("bingo\n");
                break;
            }
        }
    }
}
