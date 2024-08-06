// Method syntax
fn main() {
    // Добавляем метод структуре
    // program_1();
    // Methods with more parameters
    // program_2();
    // Associated functions
    // program_3();
    // Multiple impl blocks
    program_4();
    // Method calls are syntactic sugar for functions  calls
    // program_5();
    // Method and ownership
}

#[allow(dead_code)]
fn program_1() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    // Все методы, которые здесь прописываются, будут связаны со структурой Rectangle
    impl Rectangle {
        fn area(&self, scale: u32) -> u32 {
            self.width * self.height * scale
        }

        fn width(&self) -> bool {
            self.width > 0
        }

        fn height(&self) -> bool {
            self.height > 0
        }
    }

    let rec1 = Rectangle {
        width: 11,
        height: 20,
    };

    println!("{:#?}", rec1);
    println!("The are of rectangular is {} square pixels", rec1.area(1));

    if rec1.width() {
        println!("The rectangle has a nonzero width; it is {}", rec1.width);
    }

    if rec1.height() {
        println!("The rectangle has a nonzero width; it is {}", rec1.height);
    }
}

// Methods with more parameters
#[allow(dead_code)]
fn program_2() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self, scale: u32) -> u32 {
            self.width * self.height * scale
        }

        fn width(&self) -> bool {
            self.width > 0
        }

        fn height(&self) -> bool {
            self.height > 0
        }

        // Определение нужного метода, который принимает неизменяемое заимствование
        // на второй прямоугольник.
        fn can_hold(&self, other: &Rectangle) -> bool {
            if self.width > other.width && self.height > other.height {
                return true;
            }

            return false;
        }
    }

    let r1 = Rectangle {
        width: 100,
        height: 100,
    };

    let r2 = Rectangle {
        width: 50,
        height: 50,
    };

    let r3 = Rectangle {
        width: 30,
        height: 120,
    };

    println!("r1: {:#?}", r1);
    println!("r2: {:#?}", r2);
    println!("r3: {:#?}", r3);

    println!("Can r1 hold r2: {}", r1.can_hold(&r2));
    println!("Can r1 hold r3: {}", r1.can_hold(&r3));
}

// Associated functions
#[allow(dead_code)]
fn program_3() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self, scale: u32) -> u32 {
            self.width * self.height * scale
        }

        fn width(&self) -> bool {
            self.width > 0
        }

        fn height(&self) -> bool {
            self.height > 0
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            if self.width > other.width && self.height > other.height {
                return true;
            }

            return false;
        }

        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
        fn of(width: u32, height: u32) -> Self {
            Self { width, height }
        }
    }

    let r1 = Rectangle::of(10, 20);
    let r2 = Rectangle::square(100);

    println!("r1: {:#?}", r1);
    println!("r2: {:#?}", r2);
}

// Multiple impl blocks
#[allow(dead_code)]
fn program_4() {}

// Method calls are syntactic sugar for functions  calls
#[allow(dead_code)]
fn program_5() {}
