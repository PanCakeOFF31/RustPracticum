// Slice type
fn main() {
    // Problem - not synchronized values
    // program_1();
    // String slices
    // program_2();
    // String literals are string slices: &str
    // program_3();
    // program_4();
    // Other slices
    // program_5();
}

#[allow(dead_code)]
fn program_1() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    // Данная функция возвращает индекс пробела, который не связан и не синхронизирован
    // с реальным состоянием объекта String
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }

    s.clear();
    println!("s: {s}, index of space: {word}")
}

#[allow(dead_code)]
fn program_2() {
    let mut s = String::from("hello world");

    let hello = &s[..5];
    let world = &s[6..];

    println!("s: {s}");
    println!("hello: {hello}");
    println!("world: {world}");

    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    let first = first_word(&s);
    println!("first word: {first}");

    // После всего я выполняю clear - использую s как mutable - допустимо
    s.clear();

    // Нарушение правил, нельзя использовать immutable после mutable - нет гарантии
    // При println происходит заимствование к указанной переменной - immutable reference
    // println!("{first}");
}

#[allow(dead_code)]
fn program_3() {
    // s - является immutable reference на конкретное место двоичного файла.
    let s = "Hello world";
    println!("{s}")
    // Результат выражения строкового литерала является срезом - неизменяемая ссылка
    // на участок памяти. Строки поэтому являются неизменяемыми. Программа сама где-то объявляет
    // в памяти участок, и возвращает на него ссылку в виде среза со всей длинной
}

#[allow(dead_code)]
fn program_4() {
    let mut s = String::from("hello world");
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    let first = first_word(&s);
    println!("first word: {first}");
    s.clear();
}

// Other slices
#[allow(dead_code)]
fn program_5() {
    let mut a = [1, 2, 3, 4];
    a[3] = 15;
    println!("owner-a: {:?}", a);

    // Здесь существует только неизменяемая ссылка
    let slice = &a;
    println!("borrower-slice: {:?}", slice);

    // Здесь существует только изменяемая ссылка - прошлая удалена
    let slice = &mut a[2..];
    slice[0] = 25;
    println!(
        "mut_borrower-slice: {:?}, slice.len(): {}",
        slice,
        slice.len()
    );

    let mut my_string = String::from("Hello");
    let my_str: &mut str = (&mut my_string).as_mut();
    let my_str = &mut my_string[..];
    println!("{my_str}");
}
