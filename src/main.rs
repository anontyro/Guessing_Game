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


        match guess.cmp(&number) {
            Ordering::Less => println!("{} is lower than the number", guess),
            Ordering::Greater => println!("{} is greater than the number", guess),
            Ordering::Equal => {
                println!("{} is the correct number!", guess);
                break;
            },
        }
    }


}
