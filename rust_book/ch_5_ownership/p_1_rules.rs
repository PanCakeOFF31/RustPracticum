// Правило:
//    значение всегда у владельца
//    владелец только один
//    недействительный владелец - немедленное удаление значение
fn main() {
    // Variable scope and valid variable
    // program_1();

    // String type
    // program_2();
}

// Variable scope and valid variable
#[allow(dead_code)]
fn program_1() {
    // Scope определен фигурными скобками
    // s is not valid, s is not declared
    let s = "Maxim"; // s is valid from this and till ending scope

    {
        println!("{s}");
        let s = "Max"; // Обе s валидны, но эта s затеняет внешнюю
        println!("{s}");
    }
    // still is valid
} // scope is over, s is no longer valid

// String type
#[allow(dead_code)]
fn program_2() {
    // Это строковый литерал фиксированного размера, является неизменяемым - stack.
    let string_literal = "maxim"; // Переменная не является владельцем, она заимствует
    println!("string_literal: {string_literal}");

    // String type - хранится в куче, позволяет хранить и изменять строковые литералы.
    let mut string_type = String::from(string_literal);
    println!("string_type: {string_type}");

    string_type.push_str(" blinov");
    println!("string_type: {string_type}");
}
