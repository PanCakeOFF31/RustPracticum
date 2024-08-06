fn main() {
    // program_1();
    program_2();
    // program_3();
}

#[allow(dead_code)]
fn program_1() {
    fn call_me(num: i32) {
        for i in 0..=num {
            println!("Ring! Call number {}", i + 1);
        }
    }

    call_me(3);
}

#[allow(dead_code)]
fn program_2() {
    fn is_even(num: i64) -> bool {
        num % 2 == 0
    }
    fn sale_price(price: i64) -> i64 {
        if is_even(price) {
            price - 10
        } else {
            price - 3
        }
    }

    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

#[allow(dead_code)]
fn program_3() {
    fn square(num: i32) -> i32 {
        num * num
    }

    let answer = square(3);
    println!("The square of 3 is {answer}");
}
