use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut count = 0;
    let max_tries = 10;

    loop {
        let mut guess = String::new();
        println!("your tries left {}", max_tries - count);
        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input");

        let guess_number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please re-enter the number! ");
                continue;
            }
        };

        match guess_number.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You won!!!!");
                break;
            }
            Ordering::Less => println!("Guess Higher"),
            Ordering::Greater => println!("Guess Lower"),
        }

        count += 1;

        match count.cmp(&max_tries) {
            Ordering::Equal => {
                println!("Sorry , Better luck next time.");
                break;
            }
            Ordering::Less => continue,
            Ordering::Greater => {
                println!("Sorry, Better Luck Next time!!!");
                break;
            }
        }
    }
}
