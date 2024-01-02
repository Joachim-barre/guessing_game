use std::io;
use rand::Rng;

const DEBUG : bool = false;

fn print_diff(diff : u32){
    if diff == 0 {
        println!("You won!");
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

fn main() {
    println!("Guess the number!\nthe number if between 1 and 100");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=99);

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
                println!("Please type a number between 0 and 100!");
                continue;
            }
        };

        if guess > 99 {
            println!("Please type a number between 0 and 100!");
            continue;
        }

        println!("You guessed: {guess}");

        print_diff(guess.abs_diff(secret_number));
    
        if guess == secret_number {
            break;
        }
    }
}
