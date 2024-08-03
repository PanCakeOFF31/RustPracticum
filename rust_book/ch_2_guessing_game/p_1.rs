use std::io;

// std - standard library
// io - input/output library - sub library

// guessing game - version = "0.1.0"
fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // Создание изменяемой переменной и связывание её с пустым экземпляром String
    let mut guess = String::new();

    // stdin() -> Stdin - обработка ввода
    let bytes: usize = io::stdin()
        // вызов метода стандартной обработки ввода Stdin экземпляра
        .read_line(&mut guess) // Принимает ссылку на изменяемую guess
        // read_line возвращает Result как результат вызова
        // Возвращает в случае Err данную строку: и значение
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
    println!("byte size: {bytes}")
}
