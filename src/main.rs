extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let number = rand::thread_rng().gen_range(1,101);

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Unable to process guess");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match check_guess(number, guess) {
            true => break,
            false => continue
        };
    }

}

fn check_guess(num : u32, g: u32) -> bool {

        match g.cmp(&num) {
            Ordering::Less => {
                println!("{} is lower than the number", g);
                return false;
                },
            Ordering::Greater => {
                println!("{} is greater than the number", g);
                return false;
                },
            Ordering::Equal => {
                println!("{} is the correct number!", g);
                return true;
            },
        }
}
