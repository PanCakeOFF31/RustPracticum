fn main() {
    let message = "asd";
    println!("{}", message);

    let message = "asd".to_string();
    println!("{}", message);

    let tuple = ([1; 2], [3; 4]);
    println!("{}, {}", tuple.0[1], tuple.1[3])
}
