use rand::Rng;

// Functions: declaration, parameters, statement, expression
fn main() {
    // print_int(15);

    // Expressions: function, macros, scope block (scope)
    program_1();
}
// Функция должна аннотировать переменную типом
#[allow(dead_code)]
fn print_int(x: i8) {
    println!("x = {}", x)
}
#[allow(dead_code)]
fn program_1() {
    println!("Function call is an expression: {}", get_i32());
    println!("Scope block is an expression: {}", {
        let x = get_i32();
        x * 15
    });
    // Функция всегда возвращает пустой Tuple
    println!("Function call without return or value: {:?}", no_affect())
}
#[allow(dead_code)]
fn get_i32() -> i32 {
    // Можно вернуть через инструкцию return
    // return rand::thread_rng().gen_range(1..=100);

    // Возвращаю значение без return
    rand::thread_rng().gen_range(1..=100)
}

#[allow(dead_code)]
fn no_affect() {}

#[allow(dead_code)]
fn program_() {}
