fn main() {
    // Variable declaration
    // program_1();

    // Result.expect()...
    // program_2();

    // Result matching
    program_3();
}

// Variable declaration
fn program_1() {
    // immutable variable
    let apples = 5;
    println!("Количество яблок: {}", apples);

    // cannot assign twice to immutable variable
    // apples = 15

    // Оператор присваивания - связывает значение с переменной (binding)
    let mut oranges: i32 = 15;
    println!("Количество апельсинов до: {}", oranges);

    oranges = 10;
    println!("Количество апельсинов после: {}", oranges);

    // Передача аргументов форматирования в {} и после
    println!("apples = {apples} and oranges = {}", oranges)
}

// Enum of Result: Ok, Err
fn program_2() {
    let ok: Result<&str, &str> = Ok("Ok value");
    println!("{}", ok.expect("Error message"));

    let err: Result<u32, &str> = Err("error value");
    err.expect("Error message");
}

fn program_3() {
    fn parse_string(str: String) -> i32 {
        match str.trim().parse() {
            Ok(result) => result,
            _ => 0,
        }
    }

    let str = "10".to_string();
    println!("result: {}", parse_string(str));

    let str = "asd".to_string();
    println!("result: {}", parse_string(str));
}
