// Method syntax
fn main() {
    // Добавляем метод структуре
    // program_1();
    // Methods with more parameters
    // program_2();
    // Associated functions
    // program_3();
    // Multiple impl blocks
    // program_4();
    // Method calls are syntactic sugar for functions  calls
    // program_5();
    // Rust использует automatic referencing and dereferencing
    // program_6();
    // Method and ownership
    // program_7();
    program_8();
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
        fn new(width: u32, height: u32) -> Self {
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
fn program_4() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    // builders
    impl Rectangle {
        fn of(width: u32, height: u32) -> Self {
            Self { width, height }
        }
        fn new(width: u32, height: u32) -> Self {
            Self { width, height }
        }
    }
    impl Rectangle {
        fn compare(&self, other: &Rectangle) -> i32 {
            if self.width == other.width {
                self.height as i32 - other.height as i32
            } else {
                self.width as i32 - self.height as i32
            }
        }
    }

    let r1 = Rectangle::new(10, 20);
    let r2 = Rectangle::new(10, 30);
    let compare = r1.compare(&r2);

    if compare > 0 {
        println!("r1 больше чем r2, r1: {:?}, r2: {:?}", r1, r2);
    } else if compare < 0 {
        println!("r2 больше чем r1, r1: {:?}, r2: {:?}", r1, r2);
    } else {
        println!("r1 == r2, r1: {:?}, r2: {:?}", r1, r2);
    }
}

// Method calls are syntactic sugar for functions  calls
#[allow(dead_code)]
fn program_5() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn set_width(&mut self, width: u32) {
            self.width = width;
        }
    }

    let mut r = Rectangle {
        width: 1,
        height: 2,
    };

    let area_1 = r.area();
    let area_2 = Rectangle::area(&r);

    assert_eq!(area_1, area_2);

    println!("r: {:#?}", r);
    dbg!(r.set_width(100));
    println!("r: {:#?}", r);
    dbg!(Rectangle::set_width(&mut r, 200));
    println!("r: {:#?}", r);
}

// Rust использует automatic referencing and dereferencing
#[allow(dead_code)]
fn program_6() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn log_size(&self) {
            println!(
                "Rectangular {{width: {}, height: {}}}",
                self.width, self.height
            );
        }
    }

    let r = &mut Box::new(Rectangle {
        width: 10,
        height: 20,
    });

    // Демонстрация того, что Rust автоматически создает ссылку и разыменовывает столько
    // раз, сколько нужно, чтобы сделать типы сопоставимыми с аргументами.
    let area1 = r.area();
    let area2 = Rectangle::area(&r);
    // Mutable reference is downgraded into shared reference
    // Обратно такое сделать не позволительно.
    let area3 = Rectangle::area(&**r);

    println!("area1: {area1}");
    println!("area2: {area2}");
    println!("area3: {area3}");

    let r1 = Rectangle {
        width: 10,
        height: 20,
    };

    // Разыменование происходит до тех пор, пока тип не совпадет с &self: Self(Rectangular)
    let r1_ref = &Box::new(Box::new(Box::new(r1)));
    r1_ref.log_size();
}

// Method and ownership
#[allow(dead_code)]
fn program_7() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn set_width(&mut self, width: u32) {
            self.width = width;
        }

        fn max(self, other: Rectangle) -> Rectangle {
            Rectangle {
                width: Ord::max(self.width, other.width),
                height: Ord::max(self.height, other.height),
            }
        }

        fn set_to_max(&mut self, other: Rectangle) {
            // Такое не работает, так как у self нет разрешения на [O].
            // К (self...) обращаться за unique reference - self. То есть:
            // обращаются к владельцу, в то время, пока существует заёмщик.
            // *self = self.max(other);
        }
    }

    let rect = Rectangle {
        width: 0,
        height: 15,
    };
    println!("rect: {:#?}", rect);

    let other_rect = Rectangle {
        width: 100,
        height: 0,
    };
    println!("other_rect: {:#?}", other_rect);

    // Структура не реализует Copy, поэтому передается право
    let max_rect = rect.max(other_rect);
    println!("max_rect: {:#?}", max_rect);

    // Попытка заимствования перемещенных значений
    // println!("{:#?}", rect);
    // println!("{:#?}", other_rect);

    {
        let mut rect = Rectangle {
            width: 0,
            height: 0,
        };

        rect.set_width(10);

        let rect_ref = &rect;
        // Не может заимствовать &mut self, так как rect_ref является immut
        // rect_ref.set_width(15);

        // mut может понижаться до immut - в обратную не работает
        rect_ref.area();
    }
}

// Как программа выше, только используется Copy trait
#[allow(dead_code)]
fn program_8() {
    #[derive(Debug, Copy, Clone)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn set_width(&mut self, width: u32) {
            self.width = width;
        }

        fn max(self, other: Rectangle) -> Rectangle {
            Rectangle {
                width: Ord::max(self.width, other.width),
                height: Ord::max(self.height, other.height),
            }
        }

        fn set_to_max(&mut self, other: Rectangle) {
            // Эквивалентые две строчки
            // *self = Rectangle::max(*self, other);
            // *self = (*self).max(other);
            *self = self.max(other);
        }
    }

    let mut rect = Rectangle {
        width: 0,
        height: 15,
    };

    let other_rect = Rectangle {
        width: 100,
        height: 0,
    };

    let max_rect = rect.max(other_rect);
    println!("other_rect: {:#?}", other_rect);
    println!("rect: {:#?}", rect);
    println!("max_rect: {:#?}", max_rect);

    rect.set_to_max(other_rect);
    println!("rect: {:#?}", rect);
}
