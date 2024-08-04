// Ownership
fn main() {
    // Safety is the absence of undefined behavior
    // program_1();

    // Transfer value with copying vs not copying
    // program_2();
    // deep copy instead of move
    program_3();
}

// Interpreter vs compiler
#[allow(dead_code)]
fn program_1() {
    // Такой код не безопасен - в Rust он даже не скомпилируется
    // В интерпретируемых языках - возникнет ошибка во время выполнения
    // Cannot find value `x` in this scope
    // read(x);
    let x = false;
    read(x);
    fn read(y: bool) {
        if y {
            println!("y is true")
        } else {
            println!("y is false")
        }
    }
    // Скомпилированный Rust code - код, который по мнению компилятора удовлетворяет
    // всем требованиям безопасности.
}

// COPY vs MOVE
#[allow(dead_code)]
fn program_2() {
    let _a = [5; 10];
    let _b = _a; // COPYING
    println!("a: {:?},\nb: {:?} ", _a, _b);

    let _c = Box::new([5; 10]);
    let _d = _c; // copying of the pointer, removing of _c -> MOVING
    println!("d: {:?} ", _d);
}
// deep copy instead of move
#[allow(dead_code)]
fn program_3() {
    let first = String::from("Ferris");
    println!("first: {first}");

    let full = add_suffix(first.clone()); // ownership moving
    println!("full: {full}, original: {first}");

    fn add_suffix(mut name: String) -> String {
        name.push_str(" Jr.");
        name
    }
}
