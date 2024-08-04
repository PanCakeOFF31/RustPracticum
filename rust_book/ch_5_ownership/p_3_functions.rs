// Ownership and functions
fn main() {
    // Moving and copying
    // program_1();
    // Return value and scope
    // program_2();
    // Возврат нескольких значений
    program_3();
}

#[allow(dead_code)]
fn program_1() {
    let s = String::from("any text");
    let x = 5;

    takes_ownership(s); // Передача указателя на данные в heap - MOVE
    makes_copy(x); // Передача значения в stack - COPY

    // println!("{s}"); // borrow of moved value

    fn takes_ownership(value: String) {
        println!("Функция забрала владение объектом: {value}")
    } // value вместе с данными удаляются

    fn makes_copy(value: i32) {
        println!("Функция выполнила копирование значение: {value}")
    } // value вместе с данными удаляется, но для исходной переменной x все равно
}

// Return value and scope
#[allow(dead_code)]
fn program_2() {
    let s1 = gives_ownership();
    let s2 = String::from("hell");
    let s3 = takes_then_gives_ownership(s2);

    println!("s1: {s1}");
    println!("s2 is not valid longer");
    println!("s3: {s3}");

    fn gives_ownership() -> String {
        String::from("Any text")
    }

    // Принимает владения, а затем возвращает обратно
    fn takes_then_gives_ownership(mut value: String) -> String {
        value.push_str(" text");
        value
    }
}

// Возврат нескольких значений
#[allow(dead_code)]
fn program_3() {
    let s1 = String::from("some text");
    let (s2, s1_length) = calculate_length(s1);

    println!("s1: {s2}, s1.length: {s1_length}");
    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len();
        (s, length) // Если написать s.len() здесь, то s уже передан во владение tuple
    }
}
