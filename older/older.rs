//
use std::cmp::Ordering;
use std::io;

use rand::Rng;

// импортирование библиотеки вводы-вывода из scope - std

fn main() {
    // A guessing game
    // program_1();
    // Variables and mutability
    // program_2();
    // Immutable and mutable variables
    // program_3();
    // Shadowing
    // program_4();
    // Type annotation
    // program_5();
    // Data types
    program_6();
}
#[allow(dead_code)]
fn program_1() {
    // placeholder - заполнитель
    let x: u64 = 10;
    println!("x = {}", x);
    println!("x = {x}");
    println!("Рандомной число: {}", rand::thread_rng().gen_range(1..10));

    // println!() - использование макрокоманды
    println!("Угадай число!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("Секретное число равно {secret_number}");

    loop {
        println!("Введите свою догадку!");

        // let - используется для объявления переменной
        // mut - переменная является изменяемой
        // введение изменяемой переменной
        let mut guess: String = String::new();

        // std::io::stdin()...
        // read_line() - все что набрано в консоль помещаается в экземпляр String
        io::stdin()
            .read_line(&mut guess)
            .expect("Не получилось прочитать строку");

        // let guess: u32 = guess.trim().parse()
        //     .expect("Пожалуйста, наберите число!");
        let guess: u32 = match guess.trim().parse() {
            Result::Ok(num) => num,
            Result::Err(_) => {
                println!("Вы ввели не число!!!");
                continue;
            }
        };

        println!("Вы загадали: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Слишком малое число!"),
            Ordering::Greater => println!("Слишком большое число!"),
            Ordering::Equal => {
                println!("Вы выйграли!");
                break;
            }
        }
    }
}

#[allow(dead_code)]
fn program_2() {
    let x = 5;
    println!("Значение x равно {}", x);

    // x = 6;
    // println!("Значение x равно {}", x);

    let mut y = 5;
    println!("Значение y равно {}", y);

    y = 6;
    println!("Значение y равно {}", y);
}
#[allow(dead_code)]
fn program_3() {
    // Требуется обязательное явное указание типа переменной
    const MAX_POINTS: i32 = 100_000;
}

// Внутриблочное затенение переменной
#[allow(dead_code)]
fn program_4() {
    let x = 5;
    println!("Значение x = {x}");

    let x = x * 2;
    println!("Значение x = {x}");

    let x: i64 = x * 3;
    println!("Значение x = {x}");

    let spaces = "  ";
    let spaces = spaces.len();
    print!("{spaces}");

    // let mut exp = " ";
    // exp = exp.len();
}

#[allow(dead_code)]
fn program_5() {
    // Здесь требуется указывать аннотацию типа
    let guess: i32 = "42".parse().expect("asd");
    println!("{}", guess)
}

#[allow(dead_code)]
// Data types
fn program_6() {}
