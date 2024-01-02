use std::io;
use rand::Rng;

const DEBUG : bool = false;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    if DEBUG {
        println!("debug : secret number {secret_number}");
    }

    loop {
        println!("Your Number : ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        println!("You guessed: {guess}");

        let diff = secret_number.abs_diff(guess);
       
        if diff == 0 {
            println!("You won!");
            break;
        }else if diff < 5 {
            println!("You are really close!");
        }else if diff < 10 {
            println!("You are close!");
        }else if diff < 20 {
            println!("You are close, but a little far!");
        }else if diff < 40 {
            println!("You are a little bit too far!");
        }else {
            println!("You are really far!")
        }
    }
}
