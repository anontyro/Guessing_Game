use std::io;

fn main() {
    println!("Guess the number");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Unable to process guess");

    println!("You guess: {}", guess);
}
