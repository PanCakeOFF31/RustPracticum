use std::io;

use rand::Rng;

// Указание trait - Rng, который определяет допустимые методы с конкретным поведением

// guessing game - version = "0.2.0"
// Generating a secret number
fn main() {
    println!("Guess the number!");
    // thread_rng - возвращает ThreadRng экземпляр для генерации случайных чисел
    // gen_range - метод генерирует значение в указанном диапазоне
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // 1..100 -> [1;100), 1..=100 -> [1;100]
    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
