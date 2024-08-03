// Присваивание константной переменной - constant expression
const STARS: i32 = 10 * 10 * 20;

// Variables and mutability, constants
fn main() {
    // immutable variable - нельзя изменить связанное значение
    let x = 5;
    println!("x: {x}");
    // cannot assign a new value to an immutable variable more than once
    // x = 10;

    // mutable variable - можно пере присвоить значение
    let mut y = 15;
    println!("y: {y}");
    y = 25;
    println!("y: {y}");

    program_1();
    // К константе можно обратиться до ее объявления
    println!("Сокрытие глобальной константы STARS: {STARS}");
    const STARS: u8 = 5;
}

fn program_1() {
    println!("Глобальная константа STARS: {STARS}");
}
