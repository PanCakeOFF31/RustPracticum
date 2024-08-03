// Shadowing,
fn main() {
    let x: i32 = 5;
    println!("Immutable variable x: i32 = {x}");

    // Затеняем с изменением: типа, значения, модификатор
    let mut x: i64 = 15;
    println!("Mutable variable x: i64 = {x}");

    x = 155;
    println!("Mutable variable x: i64 = {x}");

    {
        // Могу обратиться к одноименной x, которая объявлена в enclosing scope
        let x = x * 2;
        println!("x = x * 2 -> {x}");
        // Затенение в блоке кода, изменяем: тип, значение, модификатор
        let x = "text";
        println!("Immutable variable x: &str = {x}")
    }

    println!("Mutable variable after block-scope: i64 = {x}");

    // Демонстрация разницы между mut и shadowing
    program_1();
}
// Демонстрация разницы между mut и shadowing
fn program_1() {
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces: {spaces}");

    // Выполнение кода приведет к ошибке компиляции. Разница между типами usize и &str
    // let mut spaces = "    ";
    // spaces = spaces.len();
}
