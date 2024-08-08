fn main() {
    // program_1();
    // program_2();
    program_3();
}

// Парадокс для меня, нужны ответы, никто не владеет объектом, но можно создавать ссылку
#[allow(dead_code)]
fn program_1() {
    // Код работает корректно, потому что компилятор может гарантировать безопасность.
    // Компилятор может сделать вывод о том, что значения будут жить достаточно долго
    // Значение справа живет достаточно долго. Мы не получаем висячей ссылки.
    let s = &String::from("asd");
    println!("{s}");

    let b = &Box::new(10);
    println!("{b}");
}

#[allow(dead_code)]
fn program_2() {
    let _arr = [Box::new(10), Box::new(10), Box::new(10)];
    // Нельзя переместить данные, когда они являются частью составного типа и Drop
    // let a = _arr[0];

    let mut _arr = [Box::new(10), Box::new(10), Box::new(10)];
    // Использование unique reference забирает все разрешения
    let a_ref = &mut _arr;
    println!("{a_ref:?}");
}
#[allow(dead_code)]
fn program_3() {
    let mut a = (10, 20, 30);
    let a_ref = &mut a;

    println!("{:#?}", a_ref);
}
