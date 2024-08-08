use crate::breakfast::Breakfast;
mod breakfast;

fn main() {
    program_1();
    // program_2();
    // program_3();
}

#[allow(dead_code)]
fn program_1() {
    let mut meal = Breakfast::summer("Rye");
    println!("{meal:?}");

    meal.toast = String::from("Wheat");
    println!("{meal:?}");

    Breakfast::change_season_fruit(&mut meal, "Bread");
    println!("{meal:?}");
}

#[allow(dead_code)]
fn program_2() {
    let _color_1 = breakfast::Colors::Blue;
    let _color_2 = breakfast::Colors::Red;
}

#[allow(dead_code)]
fn program_3() {}
