use std::fmt;

// Structs
// Структура - пользовательский тип данных, которой позволяет упаковать вместе связанные
// значения по смыслу и именовать их. Манипулирование структурой как одним значением.
fn main() {
    // Terms:
    // Struct - шаблон типа, который определяет имена полей, их типы и количество.
    // Instance - конкретный объект, который заполнит поля значениями в соответствии с шаблоном.
    // Associated functions -
    //

    // A struct definition
    // program_1();

    // Using the field init shorthand
    // program_2();

    // Creating instance from other instances
    // program_3();

    // Tuple structs without named fields
    // program_4();

    // Unit-like structures without  any fields
    // program_5();

    // Ownership of struct data
    // program_6();

    // Borrowing fields of a struct
    program_7();
}

// A struct definition
#[allow(dead_code)]
fn program_1() {
    // A User struct definition
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // Creating an immutable instance of the User struct
    let user1 = User {
        active: true,
        username: String::from("John"),
        email: String::from("john@gmail.com"),
        sign_in_count: 0,
    };

    //
    let user2 = (
        true,
        String::from("John"),
        String::from("john@gmail.com"),
        0,
    );

    // Для обращения к полям требует точечная нотация с названием поля
    println!(
        "user1: {}, {}, {}, {}",
        user1.active, user1.username, user1.email, user1.sign_in_count
    );

    // Для обращения к полям требует точечная нотация с порядковым номером
    println!("user2: {}, {}, {}, {}", user2.0, user2.1, user2.2, user2.3);
}

// Фабричная функция +  shorthand
#[allow(dead_code)]
fn program_2() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    fn build_user(username: String, email: String) -> User {
        User {
            active: false,
            username,
            email,
            sign_in_count: 0,
        }
    }

    let user1 = build_user(String::from("Max"), String::from("max@yandex.ru"));
    println!("user1.name: {}", user1.username);

    let mut user2 = build_user(String::from("asd"), String::from("bolist@yandex.ru"));
    println!("user2.name: {}", user2.username);

    user2.username = String::from("Bolist");
    println!("user2.name: {}", user2.username);
}

// Creating instance from other instances
#[allow(dead_code)]
fn program_3() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    impl fmt::Display for User {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "User {{ active: {}, username: {}, email: {}, sign_in_count: {}}}",
                self.active, self.username, self.email, self.sign_in_count
            )
        }
    }

    let user1 = User {
        active: false,
        username: String::from("Pavel"),
        email: String::from("pavel@yandex.ru"),
        sign_in_count: 10,
    };

    println!("{}", user1);
    // Ручной способ создания структуры на основе существующей
    let user2 = User {
        active: user1.active,
        username: String::from("petr"),
        email: user1.email,
        sign_in_count: user1.sign_in_count + 20,
    };

    // Если попытаться обратиться к user1, то: borrow of partially moved value:
    // println!("{}", user1);

    println!("{}", user2);

    // Полное копирование - update syntax
    let user3 = User { ..user2 };
    println!("{}", user3);
}

// Tuple structs without named fields
#[allow(dead_code)]
fn program_4() {
    // Определение первого типа - Color
    struct Color(i32, i32, i32);
    // Определение второго типа - Point
    struct Point(i32, i32, i32);
    // Картежная структура разными типами
    struct Mix(i32, Box<i32>, [i32; 1]);

    // Переменные black_color и original_point имеют разные типы
    let black_color: Color = Color(0, 0, 0);
    let original_point: Point = Point(0, 0, 0);
    // tuple имеет так же тип отличный от Color и Point
    let tuple = (0, 0, 0);

    println!(
        "black_color: ({}, {}, {})",
        black_color.0, black_color.1, black_color.2
    );
    println!(
        "original_point: ({}, {}, {})",
        original_point.0, original_point.1, original_point.2
    );

    println!("original tuple: {tuple:?}");

    let mix = Mix(10, Box::new(10), [10]);
    println!("mix: {}, {}, {:?}", mix.0, mix.1, mix.2);
}

// Unit-like structures without  any fields
#[allow(dead_code)]
fn program_5() {
    struct AlwaysEqual;
    let _subject = AlwaysEqual;
}

// Ownership of struct data
#[allow(dead_code)]
fn program_6() {
    // Поле name использует заимствующий тип, что требует указателя времени жизни
    struct Person {
        // name: &str
    }

    // Поле name использует владеющий тип
    struct User {
        name: String,
    }
}

// Borrowing fields of a struct
#[allow(dead_code)]
fn program_7() {
    struct Point {
        x: i32,
        y: i32,
    }

    let mut p = Point { x: 0, y: 0 };

    let x = &mut p.x;

    // Не работает
    // println!("{p}");
    // println!("{}", p.x);

    // Все работает, p.y имеет все те же разрешения [RWO]
    p.y += 144;
    println!("{}", p.y);
    println!("{x}");
}
