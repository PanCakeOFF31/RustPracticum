// Структура с открыты полем toast и закрытым полем season_fruit
#[derive(Debug)]
pub struct Breakfast {
    // открытое поле
    pub toast: String,
    // закрытое поле
    season_fruit: String,
}

impl Breakfast {
    // Так как структура имеет приватно поле, то мы должны создать ассоциированную функцию,
    // которая бы создавала структуру. Если бы не было функции summer, то мы бы не
    // могли создать структуру!
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            season_fruit: String::from("peaches"),
        }
    }

    pub fn change_season_fruit(breakfast: &mut Breakfast, str: &str) {
        breakfast.season_fruit = str.to_string();
    }
}

// У перечисления такие же модификаторы доступа как и у самого Enum
pub enum Colors {
    Blue,
    White,
    Red,
}
