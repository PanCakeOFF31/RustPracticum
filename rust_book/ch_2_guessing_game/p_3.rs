use std::cmp::Ordering;
// std::cmp::Ordering - добавляет новый тип Ordering из std::cmp
use std::io;

use rand::Rng;

// guessing game - version = "0.3.0"
// Comparing the guess to the secret number
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // shadowing the guess variable
        // type annotation needed: например u32
        let guess: u32 = guess.trim().parse().expect("Please type a number");

        // match - выражение состоит из 3 веток, каждая ветка состоит из шаблона => инструкции
        // Ordering - enum: Less, Equal, Greater
        match guess.cmp(&secret_number) {
            // match - сопоставляет результат Ordering
            // cmp - метод сравнения, вызывается для всего, что может быть сравнимо
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        }

        println!("You guessed: {}", guess);
    }
}
