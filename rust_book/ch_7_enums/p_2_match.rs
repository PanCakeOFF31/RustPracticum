fn main() {
    // match
    // program_1();

    // match and extracting values
    // program_2();

    // Matching with Option<T>
    // program_3();

    // Matches are exhaustive
    // program_4();

    // Catch-all pattern
    program_5();
}

// match
#[allow(dead_code)]
fn program_1() {
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    let coin_1 = Coin::Penny;
    let coin_2 = Coin::Quarter;

    println!("coin_1: {}", value_in_cents(&coin_1));
    println!("coin_2: {}", value_in_cents(&coin_2));

    fn value_in_cents(coin: &Coin) -> u8 {
        // match - ключевое слово, за которым пишется выражение с любым типом
        match coin {
            // coin - знание сравнивается с каждым pattern
            // Слева шаблон => Справа код выполнения
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
            // Выполнение завершается, когда находится соответствие шаблону
        }
    }
}

// match and extracting values
#[allow(dead_code)]
fn program_2() {
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    let coin = Coin::Quarter(UsState::Alaska);
    println!("coin: {}", value_in_cents(&coin));

    fn value_in_cents(coin: &Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {state:?}");
                25
            } // Coin::Quarter(state) => {
              //     match state {
              //         UsState::Alaska => 1,
              //         UsState::Alabama => 2
              //     }
        }
    }
}
// Matching with Option<T>
#[allow(dead_code)]
fn program_3() {
    // Функция принимает Option<i32>.
    // Если есть значение вернуть Some с изменением.
    // Если нет значения вернуть None
    fn plus_ten(x: &Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_ten(&five);
    let none = plus_ten(&None);

    println!("five: {five:?}");
    println!("six: {six:?}");
    println!("none: {none:?}");
}

// Matches are exhaustive
#[allow(dead_code)]
fn program_4() {
    // Шаблоны требуют покрытия всех возможных вариантов
    fn plus_one(x: &Option<i32>) {
        match x {
            Some(_) => {}
            // Ругается, если не охватить все возможные значения
            // Wildcard
            _ => {}
        }
    }
}

// Catch-all pattern and exhaustiveness requirement
#[allow(dead_code)]
fn program_5() {
    let number = 10;

    // match с универсальным шаблоном
    // Этот код удовлетворяет требованиям исчерпывающей полноты
    match &number {
        5 => {}
        10 => {}
        // catch-all pattern - если ни один не подошел шаблон, то other примет значение
        // Универсальный шаблон
        other => {
            println!("{number}, {other}")
        }
    }

    // Этот код удовлетворяет требованиям исчерпывающей полноты
    // Так как мы явно игнорируем все остальные значения, которые не удовлетворяют паттерны
    match &number {
        5 => {}
        10 => {}
        // wildcard pattern -
        _ => {}
    }
}
