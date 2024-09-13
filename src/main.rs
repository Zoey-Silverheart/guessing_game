use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess");
    let mut guess = String::new();
    let banana: i8 = 5;
    let mut apple: i8 = 8;
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line!");
    println!("You guessed: {}", guess);
    let mut gen = rand::thread_rng();
    let number: i32 = gen.gen_range(0..=10);
    println!("The number is: {}", number);
}
