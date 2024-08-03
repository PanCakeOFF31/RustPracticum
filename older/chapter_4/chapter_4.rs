// use std::ops::Add;

mod free_practice;
mod reference_borrowing;

const PRINT_SECTION: &str = "=====================================";

// Ownership
fn main() {
    log_info("crate::main::main");
    // Variable scope
    program_1();
    // String type
    program_2();
    // variables and data interacting with move
    program_3();
    // variables and data interacting with clone
    program_4();
    // ownership and functions
    program_5();
    // Return values and scope
    program_6();
    // returning ownership of some parameters
    program_7();
    // References and Borrowing
    reference_borrowing::main();
    free_practice::main();
}

fn log_info(text: &str) {
    println!("{}", PRINT_SECTION);
    println!("Выполняется {text}()")
}

pub(crate) fn log(log: &str) {
    println!("\t{log}:");
}

// Variable scope
fn program_1() {
    log_info("crate::main::program_1");

    let s: &str = "hello";

    println!("{}", s);
}

// String type
fn program_2() {
    log_info("crate::main::program_2");

    // :: - использование пространства имен
    let mut s = String::from("hello");
    println!("{s}");

    s.push_str(", world!");
    println!("{s}");

    // Функция освобождения/возврата памяти в heap
    drop(s);
}

// variables and data interacting with move
fn program_3() {
    log_info("crate::main::program_3");

    // i32 - целое число с известным фиксированным размером, значения помещаются в стек
    let _x = 5;
    let _y = _x;

    // String - сложный тип, хранится в heap, копируется лишь указатель на область памяти
    // При этом _s1 больше не имеет значение указателя,
    // переменная _s1 обладает недействительным значением
    let _s1 = String::from("hello");
    let _s2 = _s1;

    // Возникает ошибка -
    // println!("{_s1}");
    println!("{_s2}");
}

// variables and data interacting with clone
fn program_4() {
    log_info("crate::main::program_4");

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = \"{s1}\", and s2 = \"{s2}\"");
}

// ownership and functions
fn program_5() {
    // Скалярный (простой тип)
    let a = 10;
    //  Передача приведет к Copy
    let result = pow(a, 2);
    println!("a = {a}, result = {result}");

    let s = String::from("hello");
    let result = add_world(s);

    // Здесь будет ошибка - правила владения объектом не позволяют использовать
    // неопределенную переменную
    // println!("{s}");
    println!("{result}");
}

fn pow(a: i32, b: i32) -> i32 {
    a * b
}

fn add_world(word: String) -> String {
    // word входит в область видимости (Move)
    word + "world"
    // word выходит из области видимости - вызывается drop
}

// Return values and scope
fn program_6() {
    // _s1 принимает владение значением "hello"
    let _s1 = gives_ownership();
    let s2 = String::from("hello");
    // s2 передается во владение функции, а затем возвращается из ее владения.
    let _s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    // Создание объекта String и передача владения его другой переменной
    // Move
    String::from("hello")
}

fn takes_and_gives_back(text: String) -> String {
    text
}

// returning ownership of some parameters
fn program_7() {
    let s1 = String::from("maxim");
    let (s1, length) = calculate_length(s1);

    println!("Длина строки {s1} равна {length}")
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
