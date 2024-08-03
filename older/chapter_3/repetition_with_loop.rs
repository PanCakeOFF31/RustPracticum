use crate::{log_info, print_section};

pub(crate) fn main() {
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
}

// Control Flow - loop
fn program_1() {
    let mut counter: i8 = 0;
    // объявление бесконечного цикла
    loop {
        counter += 1;
        println!("Выполнилось {} раз", counter);

        if counter >= 10 {
            break;
        }
    }
}

// Control Flow - returning value from loop
fn program_2() {
    let mut counter = 0;

    let result: i32 = loop {
        counter += 1;
        println!("Цикл выполнился {counter} раз");

        if counter >= 5 {
            break counter * 10;
        }
    };

    println!("result = {}", result)
}

// Control Flow - loop label
fn program_3() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

// Control Flow - condition loop with while
fn program_4() {
    log_info("program_4");

    let mut number: i32 = 3;

    while number != 0 {
        println!("number = {number}");
        number -= 1;
    }

    println!("Цикл завершен!")
}

// Control Flow - looping through a collection with "for"
fn program_5() {
    log_info("program_5");

    let arr: [i32; 5] = [10, 20, 30, 40, 50];
    let mut index: usize = 0;

    while index < arr.len() {
        println!("Значение массива по индексу [{index}] равно {}", arr[index]);
        index += 1;
    }

    for element in arr.iter() {
        println!("Значение массива по равно {element}");
    }

    for value in 10..20 {
        // "{}" - строка форматирования
        println!("{}", value)
    }
}
