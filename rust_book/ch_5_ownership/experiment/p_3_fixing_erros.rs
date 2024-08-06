use std::rc::Rc;

// Fixing ownership errors
fn main() {
    // Returning a reference to a data.
    // Data must outlive all of its references
    // program_1();

    // Not enough permissions
    // program_2();

    // Aliasing and mutating a data structure
    // program_3();

    // Copying vs moving out of a collection
    // program_4();

    // Mutating different tuple fields
    // program_5();

    // Mutating different array elements
    // program_6();
    // program_7();
}

// Returning a reference to a data
#[allow(dead_code)]
fn program_1() {
    // Нарушение правила: данные должны пережить все их ссылки
    // Если нужно вернуть ссылку на String, то нужно будет уверенным, что
    // объект String проживет дольше чем его ссылки
    // fn return_a_string() -> &String {
    //     let s = String::from("Hello word 2016");
    //     return &s;
    // }

    // Способы решения проблемы:
    // 1) Передать право собственности на String из функции
    fn return_a_string_1() -> String {
        let s = String::from("Hello word 2016");
        s
    }

    // 2) Вернуть строковый литерал, который живет вечно
    fn return_a_string_2() -> &'static str {
        "Hello word 2016"
    }

    // 3) Использовать указатель с подсчетом ссылок - runtime checker
    fn return_a_string_3() -> Rc<String> {
        let s = Rc::new(String::from("Hello word 2016"));
        Rc::clone(&s)
    }

    // 4) Вызывающая сторона предоставляет слот для размещения строки
    fn return_a_string_4(output: &mut String) {
        output.replace_range(.., "Hello word 2016");
    }
}

// Not enough permissions
#[allow(dead_code)]
fn program_2() {
    // fn stringify_name_with_title(name: &Vec<String>) -> String {
    //     name.push(String::from("Esq."));
    //     let full = name.join("");
    //     full
    // }

    // Способы решение проблемы:
    // 1) Самый простой - использовать mut ссылку
    // Но функция не должна изменять входные данные, если вызывающая сторона не ожидает
    fn stringify_name_with_title_1(name: &mut Vec<String>) -> String {
        name.push(String::from("Esq."));
        let full = name.join("");
        full
    }

    // 2) Передача владения
    // Очень редко, когда функция берет во владение структуру данных, принадлежащую куче
    fn stringify_name_with_title_2(mut name: Vec<String>) -> String {
        name.push(String::from("Esq."));
        let full = name.join("");
        full
    }

    // 3) Клонирование входных данных
    fn stringify_name_with_title_3(name: &Vec<String>) -> String {
        let mut cloned_name = name.clone();
        cloned_name.push(String::from("Esq."));
        let full = name.join("");
        full
    }

    // 4) Моя версия - в конце главы такой же пример оказался, ахаха...
    fn stringify_name_with_title_4(name: &Vec<String>) -> String {
        let mut full = name.join("");
        full.push_str("Esq. ");
        full
    }

    let _s = String::from("asd");
    // let ss = *_s;

    let mut v: Vec<f64> = vec![1.4, 2.5, 3.6];
    for i in &mut v {
        *i += i.round();
    }

    println!("{:?}", v);
}

// Aliasing and mutating a data structure
#[allow(dead_code)]
fn program_3() {
    // Изменение структуры, когда имеется псевдоним:
    // изменяемая и неизменяемая ссылка
    // fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    //     // largest - immutable alies - removes the [W] permissions
    //     let largest = dst.iter().max_by_key(|s| s.len()).unwrap();
    //
    //     // Это программа не безопасна, так как largest может ссылаться на
    //     // deallocated данные, которые в результате push могут деаллоцировались
    //     for s in src {
    //         if s.len() > largest.len() {
    //             dst.push(s.clone());
    //         }
    //     }
    // }

    let mut v = vec![
        String::from("max"),
        String::from("min"),
        String::from("tree"),
        String::from("back"),
        String::from("fox"),
    ];

    println!("v: {:?}", v);

    let arr = [
        String::from("fall"),
        String::from("oil"),
        String::from("garbage"),
        String::from("collection"),
    ];

    add_big_strings_3(&mut v, &arr);
    println!("v: {:?}", v);

    // Способы решения проблемы:
    // 1) Сократить время жизни largest, чтобы к моменту dst.push не существовало
    // никаких ссылок
    fn add_big_strings_1(dst: &mut Vec<String>, src: &[String]) {
        // largest - immutable alies - removes the [W] permissions
        let largest = dst.iter().max_by_key(|s| s.len()).unwrap().clone();

        // Это программа не безопасна, так как largest может ссылаться на
        // deallocated данные, которые в результате push могут деаллоцировались
        for s in src {
            if s.len() > largest.len() {
                dst.push(s.clone());
            }
        }
    }

    // 2) Выполнить изменения после того, как была найдена нуж
    fn add_big_strings_2(dst: &mut Vec<String>, src: &[String]) {
        let largest = dst.iter().max_by_key(|s| s.len()).unwrap();

        let to_add: Vec<String> = src
            .iter()
            .filter(|s| s.len() > largest.len())
            .cloned()
            .collect();

        dst.extend(to_add);
    }

    // 3) Просто скопировать длину строки
    fn add_big_strings_3(dst: &mut Vec<String>, src: &[String]) {
        let largest_len = dst.iter().max_by_key(|s| s.len()).unwrap().len();

        for s in src {
            if s.len() > largest_len {
                dst.push(s.clone());
            }
        }
    }
}
// Copying vs moving out of a collection
#[allow(dead_code)]
fn program_4() {
    let v = vec![1, 10, 100];
    let n_ref = &v[0];
    // Присваивание i32 приводит к копированию
    let _n = *n_ref;

    let v = vec![String::from("Hello word 2016")];
    let _v_ref = &v[0];
    // Присваивание String - разыменованное значение из ссылки приводит к Move
    // У ссылки [O] разрешение - на право передачи собственности отсутствует
    // let s = *_v_ref;

    // Если значение не владеет данными в куче, то оно может быть скопировано без move
    // i32 - не владеет данными в куче - Copy
    // String - владеет данными в куче - Move
    // &String - заимствует, данные из кучи, то есть не владеет.

    // Исключение: изменяемая ссылка при копировании перемещается, не копируется
    let mut n = 0;
    let a = &mut n;
    let _b = a;

    // Решения проблемы с доступом к значению через ссылку
    // 1) Избегать операций передачи владения
    let v = vec![String::from("Hello word 2016")];
    let s_ref = &v[0];
    println!("{s_ref}");

    // 2) Скопировать данные, если нужна передача владения
    let v = vec![String::from("Hello word 2016")];
    let mut s = v[0].clone();
    s.push_str("!!!");
    println!("{} => {s}", &v[0]);

    // 3) Использовать специализированный метод
    let mut v = vec![String::from("Hello word 2016")];
    let mut s = v.remove(0usize);
    s.push_str("!!!");
    println!("{:?} => {s}", v);
}

// Mutating different tuple fields
#[allow(dead_code)]
fn program_5() {
    // Rust may also reject safe programs
    let mut name = (String::from("Ferris"), String::from("Rustacean"));

    let first = &name.0;
    // name : [R], name.0: [R], but name.1: [RWO]
    name.1.push_str(", Esq.");
    println!("{first} {}", name.1);

    fn get_first(name: &(String, String)) -> &String {
        &name.0
    }

    // let mut name = (String::from("Ferris"), String::from("Rustacean"));
    // Для безопасности Rust снимает разрешения на WO, как с name так и со всех элементов
    // let first = get_first(&name);
    // name: [R], name.0: [R], name.1: [RWO]
    // name.1.push_str(", Esq.");
    // println!("{first} {}", name.1);
}

// Mutating different array elements
#[allow(dead_code)]
fn program_6() {
    let mut arr = [0, 1, 2, 3, 4];

    let x = &mut arr[0];
    *x = 100;
    println!("{arr:?}");

    let mut a = [0, 1, 2, 3];
    let x = &mut a[2];

    // let _y = &a[3];
    // Нарушение правила: одновременно unique и shared references
    // *x += *_y;

    // Нарушение правил: одновременно two unique references
    // let _y = &mut a[3];
    // *x += *_y;

    // Пример реализации небезопасного кода
    let mut a = [0, 10, 100, 0];
    let x = &mut a[1] as *mut i32;
    let y = &a[2] as *const i32;

    unsafe {
        *x += *y;
    }

    println!("{a:?}");

    let mut a = [0, 10, 100, 0];
    let (a_l, a_r) = a.split_at_mut(2);
    let x = &mut a_l[1];
    let y = &a_r[0];

    *x += *y;
    print!("{a:?}")
}

#[allow(dead_code)]
fn program_7() {
    let mut v = vec![1, 2, 3];
    copy_to_prev(&mut v, 0);
    fn copy_to_prev(v: &mut Vec<i32>, i: usize) {
        let n = &mut v[i];
        // Здесь присваивается read v[i -1], зде существует только одна share reference
        // Присваивается непосредственно по адресу
        v[0] = v[i - 1];
        // Здесь проблема, так как n - unique, v.get - &self
        // *n = v.get(i - 1);
    }
}
