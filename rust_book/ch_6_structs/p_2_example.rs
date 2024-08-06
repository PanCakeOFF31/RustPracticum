// An example program using structs
fn main() {
    // Rectangular area - prototype #1 (процедурное программирование)
    // program_1();

    // Rectangular area - prototype #2 (структурное программирование)
    // program_2();

    // Rectangular area - prototype #3 (структурное программирование)
    // program_3();

    // Adding useful functionality with traits
    // program_4();
}

// Rectangular area - prototype #1 (процедурное программирование)
#[allow(dead_code)]
fn program_1() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The are of rectangular is {} square pixels",
        area(width1, height1)
    );

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
}

// Rectangular area - prototype #2 (структурное программирование)
#[allow(dead_code)]
fn program_2() {
    // Используем структуру, чтобы инкапсулировать связанные по смыслу значения.
    let rectangular1 = (30, 50);

    println!(
        "The are of rectangular is {} square pixels",
        area(rectangular1)
    );

    // Код стал удобнее, передается связанные по смыслу значения, но все равно
    // не сразу понятно что это за структура и какое значение к чему относится.
    fn area(dimensions: (u32, u32)) -> u32 {
        // Здесь происходит индексация элементов кортежа, значения менее очевидны.
        dimensions.0 * dimensions.1
    }
}

// Rectangular area - prototype #3 (структурное программирование)
#[allow(dead_code)]
fn program_3() {
    // Использование структуры добавляет больше осмысленности
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rectangular1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The are of rectangular is {} square pixels",
        area(&rectangular1)
    );

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
}

// Adding useful functionality with traits
#[allow(dead_code)]
fn program_4() {
    // Добавляем встроенные типажи за счет атрибута derive.
    // Derive генерирует код с реализацией для типажа по умолчанию
    #[derive(Clone, Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    let scale = 2;

    // dbg!() - макрос, который печатает переданное выражение и возвращается результат.
    let rectangular1 = Rectangle {
        width: dbg!(30 * scale),
        height: dbg!(50 * scale),
    };

    println!("Just rectangular1: {:?}", rectangular1);
    println!("Pretty rectangular1: {:#?}", rectangular1);
    dbg!(&rectangular1);

    let rectangular2 = rectangular1.clone();
    println!("Cloned rectangular2: {rectangular2:#?}");
}
