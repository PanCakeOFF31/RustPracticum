const PRINT_SECTION: &str = "=====================================";

pub(crate) fn log_info(text: &str) {
    println!("{}", PRINT_SECTION);
    println!("Выполняется {text}()")
}

pub(crate) fn log(log: &str) {
    println!("\t{log}:");
}
