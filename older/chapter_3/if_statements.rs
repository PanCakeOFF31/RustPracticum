use rand::Rng;

use crate::print_section;

pub(crate) fn main() {
    print_section();
    program_1();

    print_section();
    program_2();

    print_section();
    program_3();
}

// Control Flow - if else
fn program_1() {
    let number = rand::thread_rng().gen_range(0..5);
    if number > 2 && number < 10 {
        println!("Условие истинно, number = {} и number > 2", number)
    } else {
        println!("Условие ложно, number = {} и number <= 2", number)
    }
}

// Control Flow - if else if else if else
fn program_2() {
    let number = rand::thread_rng().gen_range(0..11);

    if number < 2 {
        println!("Число меньше 2, number = {}", number)
    } else if number < 4 {
        println!("Число меньше 4, number = {}", number)
    } else if number < 11 && number % 2 == 0 {
        println!("Число меньше 11 и четное, number = {}", number)
    } else {
        println!("Число меньше 11 и нечетное, number = {}", number)
    }
}

fn program_3() {
    let number = rand::thread_rng().gen_range(0..11);
    let result: i32 = if number <= 4 { 100 } else { 200 };
    println!("{} {}", number, result)
}
