// Control flow: if
fn main() {
    // if-expression with single condition and let-if
    // program_1();

    // if-expression with multiple condition
    // program_2();

    // if in let statement
    // program_3();
}
#[allow(dead_code)]
fn program_() {}

// if-expression with single condition
#[allow(dead_code)]
fn program_1() {
    let number = 15;

    let result;

    result = if number > 3 {
        println!("number > 3, number = {}", number);
        number + 10
    } else {
        0
    };

    println!("result = {}", result);

    if number != 0 {
        println!("number != 0");
    };
}

// if-expression with multiple condition
#[allow(dead_code)]
fn program_2() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
#[allow(dead_code)]
fn program_3() {
    let condition = true;
    // В конце обязательно требуется;, так как это все инструкция
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
