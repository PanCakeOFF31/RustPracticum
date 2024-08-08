fn main() {
    // if let construction
    // program_1();
    // if let else - as match
    program_2();
    // program_3();
}

// if let construction
#[allow(dead_code)]
fn program_1() {
    let config_max = Some(3u8);

    match config_max {
        // Если Some.., то что-о делаем
        Some(max) => println!("The maximum is {max}"),
        // В остальных случаях игнор
        _ => (),
    }

    // Эта конструкция демонстрирует то, что выше
    if let Some(max) = config_max {
        println!("The maximum is {max}")
    }
}

// if let else - as match
#[allow(dead_code)]
fn program_2() {
    let config_max: Option<u8> = None;

    match config_max {
        Some(max) => println!("The maximum is {max}"),
        _ => println!("Передали None"),
    }

    // if let - синтаксический сахар для match
    if let Some(max) = &config_max {
        println!("The maximum is {max}")
    } else if let None = config_max {
        println!("Передали None")
    } else {
        println!("Этот код никогда не выполнится")
    }
}

#[allow(dead_code)]
fn program_3() {}
