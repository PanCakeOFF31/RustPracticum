fn main() {
    let condition = true;
    // mut is not require
    let x;

    // Rust can determine that x is only ever assigned once
    if condition {
        x = 10;
    } else {
        x = 20;
    }
}
#[allow(dead_code)]
fn program_() {}
