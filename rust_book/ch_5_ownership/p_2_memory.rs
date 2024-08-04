use std::ptr::addr_eq;

// Memory and allocation
fn main() {
    // Interacting with move - deep copy (scalar primitive values, data from stack)
    // program_1();

    // Interacting with move - no shallow copy, move (objects)
    // program_2();

    // Interacting with clone - deep copy (heap)
    // program_3();

    // Stack-only copying (stack)
    // program_4();

    // Explicit deallocation
    // program_5();

    // Tuple: Copy vs Drop
    program_6();
}

// Interacting with move - deep copy (scalar primitive values)
#[allow(dead_code)]
fn program_1() {
    // Пример глубокого копирования
    // 5 привязывается к x
    let x = 5;
    // 5 копируется из x и привязывается к y
    let _y = x;
    // x и y хранят простые значения, которые присваиваются с копированием
}

// Interacting with move - no shallow copy, move (objects)
#[allow(dead_code)]
fn program_2() {
    // Shallow copy как в других языках не происходит, происходит move
    let s1 = String::from("hello"); // s1 is valid
    let s2 = s1; // s1 is no longer valid. s2 is valid now
                 // println!("{s1}");
    println!("{s2}");
    // По выходе из scope не будет двойного освобождения памяти для объекта String
}

// Interacting with clone - deep copy
#[allow(dead_code)]
fn program_3() {
    let s1 = String::from("hello"); // s1 is valid
                                    // deep copy of value
    let s2 = s1.clone(); // s1 is still valid. s2 is valid now

    println!("s1: {s1}, s2: {s2}");
    println!("{}", s1 == s2); // Сравнение строк
    println!("{}", s1.eq(&s2)); // Сравнивает через ==
    println!("{}", addr_eq(&s1, &s2)) // Сравнение адресов двух указателей
}

#[allow(dead_code)]
fn program_4() {
    // Значение хранится в стеке
    let x = 5;
    // Присваивание производит полную копию
    // Вызов метода производит тоже глубокую копию
    let _y = x.clone(); // Нет разницы как
    let y = x; // Нет разницы как присваивать стековое значение

    // i32 - реализует типаж Copy
    println!("x: {}, y: {}", x, y)
}

// Explicit deallocation
#[allow(dead_code)]
fn program_5() {
    let s = String::from("any text");
    drop(s); // В drop происходит передача владения
             // Borrow of moved value
             // println!("{s}");
}

// Tuple: Copy vs Drop
#[allow(dead_code)]
fn program_6() {
    let tuple = (10, String::from("text")); // Drop
                                            // let (a, b) = tuple;
                                            // let (c, d) = tuple; // d - use of moved value
    let other_tuple = tuple;
    // println!("{:?}", tuple); // tuple - borrow of moved value
}
