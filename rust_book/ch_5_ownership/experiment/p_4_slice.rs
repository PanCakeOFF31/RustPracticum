// A Slice type
fn main() {
    // A slice is a kind of reference - non-owning pointer
    // program_1();

    // String slice
    // program_2();

    // Range syntax. String literals are slices
    // program_3();

    // Other slices
    program_4();
}

// A slice is a kind of reference - non-owning pointer
#[allow(dead_code)]
fn program_1() {
    let mut string = String::from("Any string");
    let as_bytes = string.as_bytes();
    println!("{string:?} => {as_bytes:?}");

    println!("{:?}", as_bytes.iter().enumerate());

    string.replace_range(.., "М");
    println!("{string}");
}
// String slice
#[allow(dead_code)]
fn program_2() {
    let s = String::from("hello word 2016");
    // Срезы - shared-reference, при использовании slice в дело вступает borrow checker
    let hello = &s[..6];
    let word = &s[6..11];
    let _2016 = &s[12..];

    log(&s); // A reference to the entire String
    log(hello); // A reference to a portion of the String
    log(word);
    log(_2016);

    fn log(s: &str) {
        println!("s: {s}");
    }
}

// Range syntax. String literals are slices
#[allow(dead_code)]
fn program_3() {
    // s - срез на данные, которыми владеет программа - двоичный файл
    let s = "Hello, world";
}

// Other slices
#[allow(dead_code)]
fn program_4() {
    let a = [1, 2, 3, 4];
    let slices = &a[..2];

    assert_ne!(a, slices);
}
