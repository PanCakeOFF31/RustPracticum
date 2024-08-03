use crate::helpers::{log, log_info};

mod asd;
mod helpers;

fn main() {
    log_info("crate::main:main");
    // structure defining and instantiating struct
    program_1();
    // getting field, changing field
    program_2();
    // using tuple structs
    program_3();
    // using-like structs
    program_4();
}

#[derive(Debug)]
struct User {
    user_name: String,
    user_email: String,
    sign_in_count: u64,
    active: bool,
}

// structure defining
fn program_1() {
    log_info("crate::main::program_1");

    // A User struct defining

    // Creating an instance of the User struct
    let user1: User = User {
        active: true,
        user_name: "maxim".to_string(),
        user_email: "max@yandex.ru".to_string(),
        sign_in_count: 14,
    };
    println!("{:?}", user1);

    // Changing the values
    let mut user2: User = User {
        active: true,
        user_name: "denim".to_string(),
        user_email: "den@yandex.ru".to_string(),
        sign_in_count: 3,
    };

    println!("{:?}", user2);

    user2.user_name = "denchik".to_string();
    println!("{:?}", user2);

    let user3: User = get_user(String::from("slava"), String::from("slava@yandex.ru"));
    println!("{:?}", user3);

    let user4: User = user_of("slava", "slava@yandex.ru");
    println!("{:?}", user4);
}

fn get_user(name: String, email: String) -> User {
    User {
        user_name: name,
        user_email: email,
        active: true,
        sign_in_count: 0,
    }
}

fn user_of(user_name: &str, user_email: &str) -> User {
    let user_name = user_name.to_string();
    let user_email = user_email.to_string();

    User {
        user_name,
        user_email,
        active: true,
        sign_in_count: 0,
    }
}

// getting field, changing field
fn program_2() {
    log_info("crate::main::program_2");

    let user1 = user_of("max", "max@yandex.ru");
    println!("user1.user_name = {}", user1.user_name);
    println!("user1.active = {}", user1.active);
    println!("{:?}", user1);

    // Создание нового экземпляра с использованием некоторых значений
    let user2 = User {
        user_email: "bandit@banda.ru".to_string(),
        user_name: user1.user_name,
        ..user1
    };

    // String не реализует Copy trait, поэтому происходит move поля
    // и из-за этого user1 больше не действительный
    // println!("{:?}", user1);
    println!("{:?}", user2);

    log("Копирую с сохранение прежней структуры");
    let user3 = User {
        user_email: (&user2.user_email).to_string(),
        user_name: (&user2.user_name).to_string(),
        // Поля со скалярными типами реализуют Clone, их копируют
        ..user2
    };

    println!("{:?}", user2);
    println!("{:?}", user3);
}

// using tuple structs
fn program_3() {
    log_info("crate::main::program_3");
    // Объявление кортежных структур
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);
    // Создание экземпляров кортежных структур
    let black_color: Color = Color(0, 0, 0); // Кортежная структура с типом Color
    let origin_point: Point = Point(44, 15, -4); // tuple struct с типом Point
    let tuple = (15, 44, 23);

    println!("black_color: Color = {:?}", black_color);
    println!("origin_point: Color = {:?}", origin_point);
    println!("tuple: Color = {:?}", tuple);

    let f1_color = black_color.0;
    let f2_point = origin_point.1;
    let f3_tuple = tuple.2;

    println!("f1_color = {f1_color}");
    println!("f2_point = {f2_point}");
    println!("f3_tuple = {f3_tuple}");

    log("Tuple struct с Move полями");
    {
        #[derive(Debug)]
        struct Table(String, i32);

        let table_1 = Table("Bounty".to_string(), 10);
        println!("table_1 = {:?}", table_1);

        let table_name = table_1.0;
        // Владение именем стола, table_1 - undefined
        // println!("table_1 = {:?}", table_1);
        println!("table_name = {}", table_name);
    }
}

// using-like structs
fn program_4() {
    log_info("crate::main::program_4");
}
