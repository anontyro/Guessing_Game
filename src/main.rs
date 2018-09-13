extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let number = rand::thread_rng().gen_range(1,101);

    println!("Random nunber: {}",number );

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Unable to process guess");

    println!("You guess: {}", guess);
}
