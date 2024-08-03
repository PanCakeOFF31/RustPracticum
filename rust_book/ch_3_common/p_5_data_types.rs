use std::io;

// data types -> compound types: Tuple and Array
fn main() {
    // Tuple - упорядоченная коллекция элементов произвольного типа фиксированной длины
    // program_1();

    // Array - упорядоченная коллекция элементов одного типа фиксированной длины
    // program_2();

    // Демонстрация доступа к invalid index
    program_3();
}
#[allow(dead_code)]
// Tuple
fn program_1() {
    // Объявление tuple - один составной элемент
    // explicit type annotation
    let tuple_1: (i16, f32, u8) = (500, 6.4, 1);
    println!("tuple = {:?}", tuple_1);

    // pattern matching to destruct a tuple value
    let (x, y, z) = tuple_1; // destructuring
    println!("x = {}, y = {}, z = {}", x, y, z);

    // Accessing tuple elements
    println!(".0 = {}", tuple_1.0);
    println!(".1 = {}", tuple_1.1);
    println!(".2 = {}", tuple_1.2);

    let tuple_2: () = ();
    println!("{:?}", tuple_2);
}
#[allow(dead_code)]
// Array
fn program_2() {
    // Пустой массив требует явного указания типа
    let arr: [i32; 0] = [];
    println!("{:?}", arr);

    // Явное указание типа и размера
    let arr: [i32; 3] = [1, 2, 3];
    println!("{:?}", arr);

    // Размер и типа вывелись по литералам (суффикс i16)
    let arr = [1, 2i16, 3];
    println!("{:?}", arr);

    // Размер и тип вывелись по литералам (i32 - default)
    let arr = [1, 2, 3, 4, 5];
    println!("{:?}", arr);

    // Массив, инициализированный одним и тем же значением 10 раз
    let arr = [1; 10];
    println!("{:?}", arr);

    // Accessing array elements
    let arr = [15, 20];
    println!("{:?}", arr);
    println!("arr[0] = {}, arr[1] = {}", arr[0], arr[1])
}

// Демонстрация доступа к invalid index
#[allow(dead_code)]
fn program_3() {
    let a = [1, 2, 3, 4, 5];

    println!("Введите индекс массива");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number ");

    let element = a[index];
    println!("element = {}", element);
}

//
#[allow(dead_code)]
fn program_() {}
