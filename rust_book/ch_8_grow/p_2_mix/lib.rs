pub fn deliver_order() {}

pub mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        crate::lib::deliver_order();
        // super - в данном случае это модуль lib.rs
        super::deliver_order();
    }

    pub fn cook_order() {}

    pub mod deep_mod {
        pub fn do_nothing() {
            super::super::super::program_3();
        }
    }
}
