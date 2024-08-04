fn main() {
    // loop - бесконечный цикл
    // program_1();

    // innermost loop (loop in loop)
    // program_2();

    // conditional loop - while
    // program_3();

    // for
    program_4();
}
#[allow(dead_code)]
fn program_() {}

// Бесконечный цикл
#[allow(dead_code)]
fn program_1() {
    let mut counter = 0;

    // Цикл не завершится пока не будет команда break
    loop {
        counter += 1;
        if counter == 3 {
            println!("Цикл завершился на {} итерации", counter);
            break;
        };
        println!("Итерация цикла: {}", counter);
    }

    // loop-expression
    let mut counter = 0;
    let result = loop {
        counter += 5;

        if counter > 10 {
            break counter * 2;
        }
    };
    println!("loop end with result: {result}")
}

// loop label
#[allow(dead_code)]
fn program_2() {
    let mut stage = 0;

    let result = 'label: loop {
        println!("stage: {stage}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }

            if stage == 2 {
                break 'label "Super result";
            }

            remaining -= 1;
        }

        stage += 1;
    };

    println!("Все циклы завершились: {result}")
}

#[allow(dead_code)]
fn program_3() {
    let mut number = 3;

    while number >= 0 {
        println!("number: {number}");
        number -= 1;
    }

    println!("Conditional loop - while is ended!")
}

#[allow(dead_code)]
fn program_4() {
    let arr = [10, 20, 30, 40, 50];

    // Итерация по Range
    for item in 1..4 {
        println!("item: {item}");
    }

    // Итерация по массиву
    for item in arr {
        println!("array item: {item}");
    }

    // [1;3] разворачивается в [3;1]
    for item in (1..4).rev() {
        println!("item: {item}");
    }
}
