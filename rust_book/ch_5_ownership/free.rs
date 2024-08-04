//
fn main() {
    // Array Trait dependents on the type of contents
    // program_1();
    // Проверка правил заимствования
    // program_2();
    // Срез - ссылка на часть
    program_3();
}

#[allow(dead_code)]
fn program_1() {
    // Массив является объектом
    // В данном случае, тип i32 реализует Copy, поэтому происходит копирование
    let arr1 = [1, 2, 3];
    let arr2 = arr1;
    println!("{:?}; {:?}", arr1, arr2);

    // Ссылки реализуют Copy трейт, присваивание их копирует значение - указатель (на владельца)
    let s1 = "asd";
    let s2 = s1;
    println!("{s1}, {s2}");

    // Массив String - тип String, Trait: Move
    let arr1: [String; 2] = [String::from("maxim"), String::from("blinov")];
    let arr2 = arr1; // Трейт массива определился как Move

    // Макрос для каждого аргумента выполняет заимствование - создаёт ссылку
    // println!("{:?}; {:?}", arr1, arr2);

    let arr_reference: &[String] = &arr2; // Ссылка на весь массив
    let arr_slice = &arr2[0..2]; // Срез - ссылка на часть элементов (весь)
    println!("arr_reference: {:?}", arr_reference);
    println!("arr_slice: {:?}", arr_slice);
}

// Проверка правил заимствования
#[allow(dead_code)]
fn program_2() {
    // let tuple = (String::from("Tree"), "cabbage", 44, 44.5, true);
    // println!("Исходный кортеж: {:?}", tuple);

    let mut arr = ["Tree", "Cabbage", "Orange", "Berry"];
    println!("Исходный массив: {:?}", arr);

    // Чтобы иметь возможность изменять массив, он должен быть объявлен mut
    arr[3] = "Blueberry";

    {
        // Сколько угодно неизменяемых ссылок
        let ref_1 = &arr;
        let ref_2 = &arr;
        let ref_3 = &arr;
        println!("ref_1: {:?}", ref_1);
        println!("ref_2: {:?}", ref_2);
        println!("ref_3: {:?}", ref_3);
    }

    {
        // Только одна изменяемая ссылка в один момент
        // Cannot borrow `arr` as mutable more than once at a time
        let ref_1 = &mut arr;
        // let ref_2 = &mut arr;
        println!("ref_1: {:?}", ref_1);
        // println!("ref_2: {:?}", ref_2);
    }

    {
        // Нельзя смешивать изменяемые и неизменяемые ссылки
        // Cannot borrow `arr` as immutable because it is also borrowed as mutable
        let ref_1 = &mut arr;
        // let ref_2 = &mut arr;
        // let ref_2 = &arr;

        println!("ref_1: {:?}", ref_1);
        // println!("ref_2: {:?}", ref_2);
    }

    {
        // Исключение, когда можно смешивать изменяемы и неизменяемые ссылки
        let ref_1 = &arr; // Первая неизменяемая ссылка
        let ref_2 = &arr; // Первая неизменяемая ссылка
        println!("ref_1: {:?}", ref_1);
        println!("ref_2: {:?}", ref_2); // доступ до объявления - все ок

        // Изменяемая ссылка
        let ref_3 = &mut arr;
        // println!("ref_2: {:?}", ref_2); // доступ уже после объявления - нарушение правил
        println!("ref_3: {:?}", ref_3);

        // Неизменяемая ссылка
        let ref_4 = &arr; // неизменяемая ссылка - допустимо
        println!("ref_4: {:?}", ref_4);
        // ref_3[0] = "asd"; // нарушение правил - существует неизменяемая выше

        // Допустимо смешивать, изменяемые с неизменяемыми, при условии,
        // 1) доступ к неизменяемой ссылке завершается до объявления изменяемой
        // 2) запись в изменяемую ссылку завершается до объявления новой неизменяемой
    }

    {
        // Изменяемая ссылка требует от владельца быть изменяемым
        let mut arr = ["Tree", "Cabbage", "Orange", "Berry"];
        println!("Исходный массив: {:?}", arr);

        // Чтобы объявить изменяемую ссылку, объект должен быть изменяемым
        let arr_ref = &mut arr;
        println!("Исходный массив из ссылки: {:?}", arr_ref);
    }
}

// Срез - ссылка на часть
#[allow(dead_code)]
fn program_3() {
    let mut arr = [10; 10];
    println!("length: {}, arr: {:?}", arr.len(), arr);
    arr[0] = 10;

    let slice_1 = &mut arr;
    // Cannot assign to `arr[_]` because it is borrowed
    // arr[0] = 10; // Нельзя изменять данные, которые заимствуются
    slice_1[0] = 15;
    println!("length: {}, slice_1: {:?}", slice_1.len(), slice_1);

    let slice_2 = &slice_1[..(slice_1.len() / 2)];
    println!("length: {}, slice_2: {:?}", slice_2.len(), slice_2);

    let slice_3 = &&&&&&&&slice_2;
    println!("length: {}, slice_3: {:?}", slice_3.len(), slice_3);
}
