// Enum type
fn main() {
    // Defining Enum
    // program_1();
    // Enum + Struct
    // program_2();
    // Putting data directly into enum variants
    // program_3();
    // Альтернативный вариант хранения данных в Enum
    // program_4();
    // Стандартная библиотека для IpAddr
    // program_5();
    // Enum with different amounts and types
    // program_6();
    // The option enum
    program_7();
}

#[allow(dead_code)]
fn program_1() {
    // Объявление перечисления с именем IpAddrKind и множеством допустимых значений
    enum IpAddrKind {
        // Варианты перечисления относятся к пространству имён IpAddKind
        V4,
        V6,
    }

    // Двое точечная нотация - требуется для создания экземпляров
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // Сопоставления переменной с типом IpAddrKind с вариантами перечисления
    fn route(ip_addr_kind: IpAddrKind) {
        match ip_addr_kind {
            IpAddrKind::V4 => {
                println!("Это ip v4")
            }
            IpAddrKind::V6 => {
                println!("Это ip v6")
            }
        }
    }

    // Вызов функции с match expression
    route(four);
    route(six);
}

// Enum + Struct
#[allow(dead_code)]
fn program_2() {
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{:?}", home);
    println!("{:?}", loopback);
}

#[allow(dead_code)]
fn program_3() {
    // Putting data directly into each enum variant
    #[derive(Debug)]
    enum IpAddrKind {
        // Имя каждого варианта перечисления теперь функция
        V4(String),
        V6(String),
    }

    // We attach data to each variant of the enum directly
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback);
}

#[allow(dead_code)]
fn program_4() {
    // Each variant can have different types and amounts of associated data
    #[derive(Debug)]
    enum IpAddrKind {
        // Структура не позволила бы иметь разные типы ассоциированных значений
        V4(u8, u8, u8, u8),
        V6(String),
    }
}

// Стандартная библиотека для IpAddr
#[allow(dead_code)]
fn program_5() {
    struct Ipv4Addr {}

    struct Ipv6Addr {}

    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }
}

// Enum with different amounts and types
#[allow(dead_code)]
fn program_6() {
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    struct QuitMessage;
    struct MoveMessage {
        x: i32,
        y: i32,
    }

    struct WriteMessage(String);
    struct ChangeColorMessage(i32, i32, i32);

    // Определение методов
    impl Message {
        fn call(&self) {
            println!("{:?}", self);
        }
        fn still(self) {
            println!("{:?}", self);
        }
    }

    let q = Message::Quit;
    q.call();

    let m = Message::Move { x: 0, y: 0 };
    m.call();

    let w = Message::Write(String::from("Namespace"));
    w.call();

    let c = Message::ChangeColor(1, 2, 3);
    c.call();
    c.still();
    // borrow of moved value
    // println!("{:?}",c)
}

// The option enum
#[allow(dead_code)]
fn program_7() {
    // Some - сигнализирует о присутствии значения
    let some_number = Some(5);
    let some_char = Some('e');

    println!("Option value: {}", some_number.unwrap_or(0));
    println!("Option value: {}", some_char.unwrap_or('a'));

    // None - сигнализирует об отсутствии значения
    let absent_number: Option<i32> = None;
    println!("Option default value: {}", absent_number.unwrap_or(177));
}
