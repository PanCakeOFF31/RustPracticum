// data types -> scalar types - integers
fn main() {
    // Неоднозначная ситуация с type inference - explicit annotation
    // program_1()

    // Запись целочисленного литерала с суффиксом
    // program_2()

    // Переполнение битов приводит к панике
    // program_3()

    // Демонстрация signed и unsigned integer values
    program_4()

    // Запись целочисленного литерала в различной форме
    // program_5()
}

// Неоднозначная ситуация с type inference - implicit annotation
#[allow(dead_code)]
fn program_1() {
    // При парсинге возможно большое количество разных типов -> explicit type annotation
    let parsed: f64 = "42.155E+3".parse().expect("");
    println!("Parsed f64 value: {parsed}");
}

// Запись целочисленного литерала с суффиксом
#[allow(dead_code)]
fn program_2() {
    // Запись целочисленного литерала с type suffix
    let a = 32u8;
    println!("let a = 32u8, type of a: u8, a = {}", a)
    // mismatch types -> i32 не соответствует u8
    // let b: i32 = 44u8;
}

// Переполнение битов приводит к панике
#[allow(dead_code)]
fn program_3() {
    let _8_bit: i8 = 12;
    // this arithmetic operation will overflow
    let _8_bit: i8 = _8_bit * 10;
    println!("{}", _8_bit);
    // Вызов приведет к Panicked
    // let _8_bit: i8 = _8_bit * 10;
    // println!("{}", _8_bit);
}

// Демонстрация signed и unsigned integer values
#[allow(dead_code)] // использование аттрибута, чтобы компилятор не ругался
fn program_4() {
    println!("i8: i8::MIN = {}, i8::MAX ={}", i8::MIN, i8::MAX);
    println!("u8: u8::MIN = {}, u8::MAX ={}", u8::MIN, u8::MAX); /**/
    println!("i16: i16::MIN = {}, i16::MAX ={}", i16::MIN, i16::MAX);
    println!("u16: u16::MIN = {}, u16::MAX ={}", u16::MIN, u16::MAX);
    println!("i32: i32::MIN = {}, i32::MAX ={}", i32::MIN, i32::MAX);
    println!("u32: u32::MIN = {}, u32::MAX ={}", u32::MIN, u32::MAX);
    println!("i64: i64::MIN = {}, i64::MAX ={}", i64::MIN, i64::MAX);
    println!("u64: u64::MIN = {}, u64::MAX ={}", u64::MIN, u64::MAX);
    println!(
        "i128: i128::MIN = {},\n i128::MAX ={}",
        i128::MIN,
        i128::MAX
    );
    println!(
        "u128: u128::MIN = {},\n u128::MAX ={}",
        u128::MIN,
        u128::MAX
    );
    println!(
        "isize: isize::BITS = {} isize::MIN = {}, isize::MAX ={}",
        isize::BITS,
        isize::MIN,
        isize::MAX
    );
    println!(
        "usize: usize::BITS = {} usize::MIN = {}, usize::MAX ={}",
        usize::BITS,
        usize::MIN,
        usize::MAX
    );
}

#[allow(dead_code)]
// Запись целочисленного литерала в различной форме
fn program_5() {
    let decimal = 98_222;
    let hexadecimal = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0001;
    let byte = b'!';

    println!("let decimal = 98_222 --> {}", decimal);
    println!("let hexadecimal = 0xff --> {}", hexadecimal);
    println!("let octal = 0o77 --> {}", octal);
    println!("let binary = 0b1111_0001 --> {}", binary);
    println!("let byte = b'!' --> {}", byte);
}
