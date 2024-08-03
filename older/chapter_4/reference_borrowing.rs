use std::ops::Add;

use crate::{log, log_info};

pub(crate) fn main() {
    log_info("crate::references_borrowing::main");
    // using references
    program_1();
    // try to modify smt we are borrowing
    program_2();
    // many references on one variable
    // one mut reference on one mut variable
    program_3();
    // many references on one variable
    program_4();
    // dangling reference
    program_5();
    // slice type
    program_6();
    // string slice
    program_7();
    // string slice and find first
    program_8();
    // improving the first_word()
    program_9();
    // other slices
    program_10();
    // slice type repetition
    program_11();
}

// using references
fn program_1() {
    log_info("crate::main::program_1");

    let s1 = String::from("hello");
    let length = calculate_length(&s1);
    let _s2: &String = &s1;

    println!("Длина строки \"{s1}\" равна {length}");
}

fn calculate_length(s: &String) -> usize {
    // s - ссылка на экземпляр типа String
    s.len()
    // s - выходит из области видимости. Но поскольку s не владеет тем, на что ссылается,
    // ничего не происходит
}

// try to modify smt we are borrowing
fn program_2() {
    log_info("crate::main::program_2");

    let mut s = String::new().add("Hello, ");
    println!("s before foo \"{s}\"");
    foo_1(&mut s);
    println!("s after foo \"{s}\"");

    // push_str() - метод изменяет исходный объект, требуется "mut"
    let s2 = String::from("some text");
    // s2.push_str(&"some text again");
    let s3 = s2.add(" again some text");
    // println!("{s2}");
    println!("{}", s3);
}

fn foo_1(s: &mut String) {
    s.push_str("world!");
}

// many references on one variable
// one mut reference on one mut variable
fn program_3() {
    log_info("crate::main::program_3");

    // Одна переменная, много ссылок
    {
        let s1 = String::new().add("some text");
        let s1_r1 = &s1;
        let s1_r2 = &s1;
        println!("immutable references: {s1}, {s1_r1}, {s1_r2}");
    };

    // Одна изменяемая переменная - только одна ссылка в отдельной области
    {
        let mut s2 = String::new().add("some text");
        let s2_r1 = &mut s2;
        // cannot borrow more than once at a time
        // let s2_r2 = &mut s2;
        println!("mut s2: {}", s2_r1);
        println!("&mut s2_r1: {}", s2);
    };
}

// many references on one variable
fn program_4() {
    log_info("crate::main::program_4");

    // Множественны неизменяемые ссылки на отдельный фрагмент данных
    {
        let s = String::new().add("some text");

        let _r1 = &s;
        let _r2 = &s;
    }

    // Нельзя смешивать изменяемые и неизменяемые ссылки
    // Нельзя использовать mutable и immutable заимствование одновременно
    {
        let s = String::new().add("some text");

        let _r1 = &s;
        let _r2 = &s;
        // let r2 = &mut s;

        println!("{}", _r1)
    }

    // Демонстрация, новая область видимости позволяет использовать одну
    // новую изменяемую ссылку
    {
        let mut s = String::new().add("some text");
        let r1 = &mut s;
        println!("{r1}");

        {
            let r2 = &mut s;
            println!("{r2}");
        };

        println!("{s}");
    }
}

// dangling reference
fn program_5() {
    log_info("crate::main::program_5");

    // Missing lifetime specifier
    // let _reference_to_nothing = dangle();

    // Ownership передается
    let _s = dangle();
}

// Missing lifetime specifier
// fn dangle() -> &String {
//     let s = String::from("book");
//     &s
// s выйдет из области и отбросится, а ссылку будет указывать на  недействительный
// участок памяти, Rust такого не допускает.
// }
fn dangle() -> String {
    let s = String::from("book");
    s
}

// Slice type
fn program_6() {
    log_info("crate::main::program_6");

    let mut s = String::from("hello world");
    println!("{}", &s);

    let word = first_word(&s);
    println!("{}", &word);
    println!("{}", &s);

    let w = &s[word + 1..s.len()];
    println!("{}", w);

    s.clear();
    println!("{}", &s);
    println!("{}", &word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// string slice
fn program_7() {
    log_info("crate::main::program_7");

    let s: String = String::from("hello world");

    let _hello: &str = &s[0..5];
    let hello = &s[..5];
    let _world: &str = &s[6..11];
    let world: &str = &s[6..];

    let whole_word = &s[..];

    println!("\"{}\" \"{}\" \"{}\"", s, hello, world);
    println!("\"{}\"", whole_word);
}

// string slice and find first word
fn program_8() {
    log_info("crate::main::program_8");

    let mut s: String = String::from("hello world");

    let first_word = find_first_word(&s);

    println!("\"{}\" -> \"{}\"", s, first_word);

    s.clear();
    // println!("{}", first_word);
}

fn find_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}

// improving the first_word()
fn program_9() {
    log_info("crate::main::program_9");

    let mut s: String = String::from("hello world");
    // строковый литерал уже является строковым срезом - можно передать напрямую
    let first_word = find_first_improved("hello world");
    let first_word = find_first_improved(&s[..]); // здесь существует immutable ref

    println!("\"{}\" -> \"{}\"", s, first_word);
    // в работе метода передается &mut self - нарушение правила заимствования
    // в случае, если я попытаюсь обратиться к s позже
    s.clear();

    // избегаем ошибку -> гонка данных
    // println!("\"{}\" -> \"{}\"", s, first_word);
}

fn find_first_improved(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}

// other slices
fn program_10() {
    log_info("crate::main::program_10");
    
    let a = [1,2,3,4,6];
    let slice: &[i32] = &a[0..3];
    
    println!("{:?}, {:?}", a, slice);
}



// slice type repetition
fn program_11() {
    log_info("crate::main::program_11");
    
    log("");
    {
        let s = "hello world".to_string();

        let hello = &s[0..5];
        let world = &s[6..11];
    }
}