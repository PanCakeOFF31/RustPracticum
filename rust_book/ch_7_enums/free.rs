fn main() {
    // match with integer
    // program_1();
    program_2();
    // program_3();
}

// match with integer
#[allow(dead_code)]
fn program_1() {
    match_i32(&-100);
    match_i32(&0);
    match_i32(&100);
    match_i32(&101);
    match_i32(&1000);
    match_i32(&55);
    fn match_i32(length: &i32) {
        match length {
            55 => {
                println!("Особый случай, когда length = {length}")
            }
            ..=100 => {
                println!("Передано значение Range(..=100): [{length}]")
            }
            // Если убрать 101.. pattern - то паттерны не будут охватывать все возможные варианты
            101.. => {
                println!("Передано значение Range(101..): [{length}]")
            }
        }
    }
}

#[allow(dead_code)]
fn program_2() {
    let arg = Option::Some(String::from(""));
    let arg_ref = &arg;
    // let s = arg_ref.unwrap();
    // println!("{arg:?}")

    let s1 = "asd".to_string();
    let s2 = "asd".to_string();
    println!("{s1}, {s2}");
}

#[allow(dead_code)]
fn program_3() {}
