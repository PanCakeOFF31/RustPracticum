// data types -> scalar types: floats, boolean, characters
fn main() {
    // Floating-point numbers
    // program_1();

    // Numeric operations
    // program_2();

    // Boolean type
    // program_3();

    // Character type
    program_4();

    // program_5();
}

#[allow(dead_code)]
// Floating-point numbers
fn program_1() {
    let x = 2.0000006;
    let y: f32 = 3.0000001;
    println!("x: f64 = 2.00000006, x = {}", x);
    println!("y: f32 = 3.0000001, y = {}", y);
}
#[allow(dead_code)]
fn program_2() {
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let mult = 4f64 * 30.3;
    let division = 56.7 / 32.2;
    let reminder = 43 % 5;

    println!("let sum = 5 + 10 --> {}", sum);
    println!("let difference = 95.5 - 4.3 --> {}", difference);
    println!("let mult = 4f64 * 30.3 --> {}", mult);
    println!("let division = 56.7 / 32.2 --> {}", division);
    println!("let reminder = 43 % 5 --> {}", reminder);
}
// Boolean type
#[allow(dead_code)]
fn program_3() {
    let _t = true;
    let _f: bool = false;
}

// Character type
#[allow(dead_code)]
fn program_4() {
    let c = 'Z';
    let z: char = 'z'; // explicit type annotation
    let heart_eyed_cat = '😻';
    println!("char --> {}", c);
    println!("char --> {}", z);
    println!("char --> {}", heart_eyed_cat);
}
