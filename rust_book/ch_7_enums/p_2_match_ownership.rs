fn main() {
    program_1();
    // program_2();
    // program_3();
}

#[allow(dead_code)]
fn program_1() {
    let option = Option::Some(String::from("Hello"));

    match &option {
        None => {}
        // Здесь происходит копирование указателя, владение осталось у option
        Some(s) => {}
    }

    match option {
        None => {}
        // Здесь происходит передача владения в рукаве Some
        Some(s) => {}
    }
}

#[allow(dead_code)]
fn program_2() {}

#[allow(dead_code)]
fn program_3() {}
