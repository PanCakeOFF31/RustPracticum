// use создает псевдоним для Colors только для p_4.rs области модуля
use crate::parent::child::c;
use breakfast::Colors;

mod breakfast;
fn main() {
    // program_1();
    // program_2();
    // program_3();
    // Idiomatic way to bring HashMap (struct) into scope
    // program_4();
    // Nested paths to clean up large use lists
    program_5();
}

#[allow(dead_code)]
fn program_1() {
    let color = Colors::Red;
}

mod inner_use {
    // Требуется повторное использование use, так как это другая модульная область видимости
    // use crate::breakfast::Colors;
    // Второй вариант - ссылка на псевдоним в родительском модуле
    use super::Colors;

    #[allow(dead_code)]
    fn program_1() {
        let colro = Colors::Red;
    }
}

// Для включения двух типов с одинаковыми именами в одну область необходимо использовать
// пути с их родительскими модулями.
#[allow(dead_code)]
fn program_2() {
    use std::fmt;
    use std::io;

    fn fn1() -> fmt::Result {
        fmt::Result::Ok(())
    }

    fn fn2() -> io::Result<i32> {
        io::Result::Ok(10)
    }
}

// Альтернативный способ избежать конфликта имен при введении (импорте) элементов из
// разных модулей с одинаковыми именами
#[allow(dead_code)]
fn program_3() {
    use std::fmt::Result as FmtResult;
    use std::io::Result as IoResult;
    fn fn1() -> FmtResult {
        FmtResult::Ok(())
    }

    fn fn2() -> IoResult<i32> {
        IoResult::Ok(10)
    }
}

// Здесь pub use вводит имя Clr и делает его публичным. Теперь создается эффект
// будто  re_export модуль определяет перечисление Color as Clr
mod re_export {
    pub use crate::breakfast::Colors as Clr;
}

mod export {
    use super::re_export::Clr;

    #[allow(dead_code)]
    fn foo() {
        Clr::Red {};
    }
}

#[allow(dead_code)]
fn program_4() {
    // Идиоматический пример введения HashMap структуры
    use std::collections::HashMap;

    let mut map: HashMap<i32, &str> = HashMap::new();

    map.insert(10, "maxim");
    map.insert(20, "blinov");

    println!("{map:?}");
}

// Nested paths to clean up large use lists
#[allow(dead_code)]
fn program_5() {
    use std::cmp::Ordering;

    let a = io::empty();
    let b = Ordering::Greater;

    use std::collections::*;
    use std::io::{self, Write};
}

#[allow(dead_code)]
fn program_6() {
    parent::a();
    parent::child::c();

    // * дает доступ абсолютно ко всем доступным элементам на любом уровне вложенности
    use parent::{child as alias, *};
    // use child;

    a();
    child::c();
    alias::c();
    c();
}
pub mod child {
    pub fn c() {}
}
pub mod parent {
    pub fn a() {}
    fn b() {}
    pub mod child {
        pub fn c() {}
    }
}
