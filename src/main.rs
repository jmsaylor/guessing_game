use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    loop {
        println!("Guess your number...");
        println!("Input your guess! __");

        let secret_number = rand::thread_rng().gen_range(1, 101);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read - valid input?");

        let guess: u32 = guess.trim().parse().expect("Has to be a number...");

        println!("Your guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You Win!!!"),
        }
        println!("Secret number: {}", secret_number)
    }
}
