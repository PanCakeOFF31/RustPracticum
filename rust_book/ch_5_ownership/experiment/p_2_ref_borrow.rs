fn main() {
    // move-only API problem - ownership lose
    // program_1();

    // move-only API problem - return
    // program_2();

    // References are non-owning pointers
    // program_3();

    // Dereferencing a pointer
    // program_4();
    // program_5();

    // Simultaneous aliasing and mutation - disallowing
    // program_6();

    // Permissions on Paths: Read, Write, Own
    // program_7();
    program_8()
}

// move-only API problem
#[allow(dead_code)]
fn program_1() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");

    greet(m1, m2);
    fn greet(m1: String, m2: String) {
        println!("{} {}!", m1, m2)
    } // Память ассоциированная с m1, m2 высвободилась

    // Проблема, мы передали владение greet(), теперь мы не может обращаться к m1, m2
    // let s = format!("{} {}", m1, m2);
    // println!("{s}")
}

// move-only API problem - return
#[allow(dead_code)]
fn program_2() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");

    // too verbose
    let (m1, m2) = greet(m1, m2);
    fn greet(m1: String, m2: String) -> (String, String) {
        println!("{} {}!", m1, m2);
        (m1, m2)
    }

    let s = format!("{} {}", m1, m2);
    println!("{s}")
}

#[allow(dead_code)]
fn program_3() {
    // m1, m2 владеют
    let m1 = String::from("Hello");
    let m2 = String::from("world");

    // & - создает ссылки или заимствует
    greet(&m1, &m2);
    fn greet(m1: &String, m2: &String) {
        println!("{} {}!", m1, m2);
    }

    // m1, m2 - все еще владельцы значений
    let s = format!("{} {}!", m1, m2);
    println!("{s}")
}

// Dereferencing a pointer
#[allow(dead_code)]
fn program_4() {
    let mut x = Box::new(1);
    let _a = *x;
    *x += 1;
    println!("{}", x);

    let r1 = &x;
    let _b = **r1;

    let r2 = &*x;
    let c = *r2;
    println!("{c}");
}

// Dereferencing a pointer
#[allow(dead_code)]
fn program_5() {
    let x = Box::new(15);

    // explicit dereference
    let x_abs_1 = i32::abs(*x); // abs принимает переменную-указатель

    // implicit dereference
    let x_abs_2 = x.abs(); // Вызов метода

    println!("x_abs_1: {x_abs_1}, x_abs_2: {x_abs_2}");
    assert_eq!(x_abs_1, x_abs_2);

    let r = &x;

    // explicit dereference
    let r_abs_1 = i32::abs(**r); // abs принимает переменную-указатель

    // implicit dereference
    let r_abs_2 = r.abs(); // Вызов метода

    println!("r_abs_1: {r_abs_1}, r_abs_2: {r_abs_2}");
    assert_eq!(r_abs_1, r_abs_2);

    let s = String::from("Hello");
    let s_len_1 = str::len(&s);
    let s_len_2 = s.len();
    println!("s_len_1: {s_len_1}, s_len_2: {s_len_2}");
    assert_eq!(s_len_1, s_len_2);
}

#[allow(dead_code)]
fn program_6() {
    let mut vector: Vec<i32> = vec![1, 10, 100];
    println!("vector: {:?}", vector);

    vector.push(1000);
    println!("vector: {:?}", vector);

    let mut v = vec![1, 2, 3];
    let num = &v[2]; // immutable reference - alias

    v.push(4); // mutable reference - mutation

    // Ниже возникнет ошибка, нарушение правила alies and mutation at the same time
    // println!("Third element is {}", num);
}

// Permissions on Paths: Read, Write, Own
#[allow(dead_code)]
fn program_7() {
    // [RO] permission by default
    let _a = 10;
    // [RWO] permission, mu keyword allow to write
    let mut _a = 10;
    _a += 1;

    // [RWO]
    let mut v = vec![15, 30, 45];
    println!("v: {:?}", v);

    let slice = &v[2];

    // [R] - only read
    let num = slice;
    println!("num: {}", *num);

    // [RWO]
    let value = *slice;
    println!("value: {value}");
}

#[allow(dead_code)]
fn program_8() {
    let x = 0;
    let mut x_ref = &x;
}
