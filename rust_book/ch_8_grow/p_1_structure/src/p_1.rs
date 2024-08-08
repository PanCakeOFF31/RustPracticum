// Так как я указал бинарный крейт с путем к нему, то p_1_structure.rs - crate root
use crate::garden::vegetables::Asparagus;

// Корень крейта требует указания подключаемых модулей
// Объявление нового модуля
pub mod garden;
mod lib;

fn main() {
    //
    program_1();
    // program_2();
    // program_3();
}

#[allow(dead_code)]
fn program_1() {
    let f1 = garden::vegetables::Asparagus {};
    let f2 = Asparagus {};

    println!("f1: {f1:#?}, f2: {f2:?}");
}

#[allow(dead_code)]
fn program_2() {
    let a = front_of_house::School::A;
}

#[allow(dead_code)]
fn program_3() {
    let apple = lib::Apple {};
}

mod front_of_house {
    pub(crate) enum School {
        A,
    }
}
