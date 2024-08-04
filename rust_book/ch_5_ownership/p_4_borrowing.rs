// References and borrowing
fn main() {
    // Referencing to String
    // program_1();

    // Modifying of borrowed value
    // program_2();

    // Ограничения ссылок
    // program_3();

    // Dangling reference
    program_4();
}

#[allow(dead_code)]
fn program_1() {
    let s1 = String::from("hello");
    let length = calculate_length(&s1);

    // Параметры функции заимствуют значения
    fn calculate_length(s: &String) -> usize {
        println!("{}", *s);
        s.len()
    }

    // s1 продолжает оставаться владельцем объекта String
    println!("{s1}, {length}")
}
// Modifying of borrowed value
#[allow(dead_code)]
fn program_2() {
    let mut s = String::from("Ownership ");
    change_str(&mut s);

    // Ссылки по умолчанию тоже являются неизменяемыми
    fn change_str(any_str: &mut String) {
        any_str.push_str("any string");
    }

    println!("s: {s}")
}

// Reference restriction: only one mutable reference in scope
#[allow(dead_code)]
fn program_3() {
    {
        let s = String::from("Ownership ");

        // Здесь происходит создание mut переменной r_1, которой присваивается immut ссылка на s
        let mut r_1 = &s;
        r_1 = &s; // Демонстрация, присвоили другое значение

        println!("{s}, {r_1}")
    }

    // Нельзя использовать несколько изменяемых ссылок
    {
        let mut s = String::from("Ownership ");

        {
            // Здесь существует только одна изменяемая ссылка
            let r_2 = &mut s;
            println!("{r_2}")
        } // По выходе изменяемая ссылка удалится

        // Здесь существует только одна изменяемая ссылка
        let r_1 = &mut s;
        // let r_2 = &mut s; // Cannot borrow `s` as mutable more than once at a time
        // println!("{r_1}, {r_2}");
        println!("{r_1}")
    }

    // Нельзя комбинировать изменяемые и неизменяемые ссылки
    {
        let mut s = String::from("Any text");

        let r_1 = &s;
        let r_2 = &s;
        let r_3 = &s;

        // Cannot borrow `s` as mutable because it is also borrowed as immutable
        // let r_4 = &mut s;

        println!("{r_1}, {r_2}, {r_3}");

        // Здесь допустимо использовать изменяемую ссылку, так как не происходит использования
        // неизменяемых ссылок
        let r_4 = &mut s;
    }
}

// Dangling reference
#[allow(dead_code)]
fn program_4() {
    // let reference_to_nothing = dangle();
    // fn dangle() -> &String {
    //     let s = String::from("asd");
    //     return &s;
    // } // данные в s - высвобождаются, &s будет указывать на недопустимое место в памяти.
}
