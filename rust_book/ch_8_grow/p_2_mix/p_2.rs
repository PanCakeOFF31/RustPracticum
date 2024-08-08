use crate::lib::deliver_order;

mod lib;

fn main() {
    // program_1();
    // program_2();
    // program_3();
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            super::remove_from_waitlist();
        }
    }

    pub fn remove_from_waitlist() {}
}

fn eat_at_restaurant() {
    // Абсолютный путь в дереве модулей для проекта
    crate::front_of_house::hosting::add_to_waitlist();
    // Относительный путь относительно идентификатора модуля front_of_house
    // Модуль front_of_house находится на том же уровне дерева модулей, что и eat_at_restaurant
    front_of_house::hosting::add_to_waitlist();
}
#[allow(dead_code)]
fn program_1() {
    deliver_order();
}

#[allow(dead_code)]
fn program_2() {}

#[allow(dead_code)]
fn program_3() {}
