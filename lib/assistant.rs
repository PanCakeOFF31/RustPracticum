pub fn log(text: &str) {
    println!("{text}")
}

pub fn max_max(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    }

    return b;
}
