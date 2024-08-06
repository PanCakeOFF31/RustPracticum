// Ownership recap
// https://rust-book.cs.brown.edu/ch04-05-ownership-recap.html
fn main() {
    // Ownership versus Garbage collection
    // program_1();
    // The concepts of ownership
    // program_2();
    // Appropriate permission
    // program_3();
    // Connection ownership between compile-tine and runtime
    // program_4();

    // find_contains
    program_5();
}

#[allow(dead_code)]
fn program_1() {
    vec![1, 2, 3].to_vec();
}

#[allow(dead_code)]
fn program_2() {
    let mut a = 0;
    inner(&mut a);
    fn inner(x: &mut i32) {
        *x += 10i32;
    }

    println!("{a}")
}

#[allow(dead_code)]
fn program_3() {
    // [RO]
    let a = 0;
    // [RO]
    let b = a;
    let b_ref = &b;
    // У b_ref есть только [R], но так как i32 - Copy, то разрешение на [O] не требуется
    let c = *b_ref;
    println!("{b}, {c}");
}

#[allow(dead_code)]
fn program_4() {
    let mut tuple = (String::from("Asd"), String::from("Asd"));

    let t_ref_1 = &mut tuple.0;
    let t_ref_2 = &mut tuple.1;

    println!("{t_ref_1}, {t_ref_2}");
    let first = get_first(&mut tuple);
    // let second = get_first(&mut tuple);

    // println!("{first}, {second}");
    fn get_first(t: &mut (String, String)) -> &mut String {
        &mut t.0
    }
}

#[allow(dead_code)]
fn program_5() {
    let s = [
        String::from("abcdefgh"),
        String::from("abcdxcfgh"),
        String::from("abcdefgh"),
        String::from("abcdcxfgh"),
    ];

    let result = find_contains(&s, "efg");
    fn find_contains<'a>(strings: &'a [String], substring: &str) -> Vec<&'a String> {
        let mut result: Vec<&String> = vec![];

        for str in strings.iter() {
            if str.contains(substring) {
                result.push(str);
            }
        }

        return result;
    }

    println!("{s:?}");
    println!("{result:?}");
}
