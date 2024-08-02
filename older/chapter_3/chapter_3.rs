mod if_statements;
mod repetition_with_loop;

const PRINT_SECTION: &str = "==================program==================";

fn main() {
    print_section();
    program_1();

    print_section();
    program_2();

    print_section();
    program_3();

    print_section();
    program_4();

    print_section();
    program_5();

    print_section();
    program_6();

    print_section();
    program_7();

    print_section();
    program_8();

    print_section();
    program_9();

    print_section();
    program_10();

    print_section();
    program_11();

    print_section();
    program_12();

    print_section();
    program_13();

    print_section();
    program_14();

    // Control Flow
    if_statements::main();
    repetition_with_loop::main();
}

fn print_section() {
    println!("{}", PRINT_SECTION);
}

fn log_info(text: &str) {
    println!("Выполняется {text}()")
}

// Демонстрация mutable/immutable переменной
fn program_1() {
    let mut x = 5;
    println!("Значение x равное {}", x);

    x = 6;
    println!("Значение x равное {}", x);

    // Здесь будет ошибка E0384
    // let y = 32;
    // y = 4;
}

const MIN_POINTS: u128 = 0;

// constant
fn program_2() {
    const MAX_POINTS: u128 = 100_000;
    println!("Значение константы MAX_POINTS = {}", MAX_POINTS);
    println!("Значение константы MIN_POINTS = {}", MIN_POINTS);
}

// shadowing
fn program_3() {
    let spaces = "    ";
    println!("Переменная spaces содержит \"{}\"", spaces);

    let spaces = spaces.len();
    println!("Переменная spaces содержит \"{}\"", spaces);
}

// data types
fn program_4() {
    // здесь требуется явное указание типа
    let guess: u32 = "42".parse().expect("");
    println!("{}", guess);
}

// data types - integer
fn program_5() {
    let a = 1_000u64;
    println!("{}", a);

    let b_decimal = 10;
    let b_hexadecimal = 0x_10_u8;
    let b_octal = 0o_77_i16;
    let b_binary = 0b_101010;

    println!(
        "{}, {}, {}, {}",
        b_decimal, b_hexadecimal, b_octal, b_binary
    );
}

// data types - floating point
fn program_6() {
    let a = 2.1;
    let b = 2.2_f32;

    println!("{}, {}", a, b);
}

// data types - numeric operations
fn program_7() {
    let a = 27;
    let b = 14;

    println!("\"a + b\" = {}", a + b);
    println!("\"a - b\" = {}", a - b);
    println!("\"a * b\" = {}", a * b);
    println!("\"a / b\" = {}", a / b);
    println!("\"a % b\" = {}", a % b);

    println!("{}", 10.0 / 15.5);
}

// data types - boolean type
fn program_8() {
    let t: bool = true;
    const F: bool = false;

    println!("{} {}", t, F)
}

// data types - character type
fn program_9() {
    let c = 'Z';
    let z = '🐍';

    println!("{} {}", c, z)
}

// data types - compound types - tuple
fn program_10() {
    let tuple: (i32, f64, &str, char) = (500, 6.4, "Maxim", '😶');
    let clone = tuple.clone();
    let (a, b, c, d) = clone;

    println!("{}", tuple.0);
    println!("{}", clone.2);

    println!("{} {} {} {}", a, b, c, d)
}

// compound types - array
fn program_11() {
    let arr_1 = [10, 20, 30];
    println!("{:?}", arr_1);

    let arr_2: [i64; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", arr_2);

    let arr_3: [i8; 0] = [];
    println!("{:?}", arr_3);

    let arr_4: [i8; 4] = [99; 4];
    println!("{:?}", arr_4);
    println!("{}", arr_4[3]);
}

// function - parameters
fn program_12() {
    print_value(10);
    print_value(15);
}

fn print_value(value: i32) {
    println!("value = {}", value)
}

// function - statement vs expression
fn program_13() {
    let x = 5;
    let y = {
        let x = 3;
        let _ = x + 1;
        x + 10 // this is ending expression
    };

    println!("{} {}", x, y);
}

// function with return value - ending expression
fn program_14() {
    println!("{}", five());
    println!("{}", sum(10, 15));
}

/// Rust Documentation comment
fn five() -> i32 {
    5
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}
