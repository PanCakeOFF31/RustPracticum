fn main() {
    // program_1();
    // program_2();
    // program_3();
}

#[allow(dead_code)]
fn program_1() {}
#[allow(dead_code)]
fn program_2() {}

#[allow(dead_code)]
fn program_3() {}

// Task - #1

#[allow(dead_code)]
fn bigger(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    }

    return b;
}
#[cfg(test)]
mod tests_bigger {
    use crate::bigger;

    #[test]
    fn t_0010_ps_() {
        assert_eq!(10, bigger(10, 8));
    }
    #[test]
    fn t_0011_ps_() {
        assert_eq!(10, bigger(8, 10));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}

// Task - #2
#[allow(dead_code)]
fn foo_if_fizz(fizzish: &str) -> &str {
    if fizzish == "fizz" {
        "foo"
    } else if fizzish == "fuzz" {
        "bar"
    } else {
        "baz"
    }
}

#[cfg(test)]
mod tests_foo_for_fizz {
    use super::*;

    #[test]
    fn foo_for_fizz() {
        assert_eq!(foo_if_fizz("fizz"), "foo");
    }

    #[test]
    fn bar_for_fuzz() {
        assert_eq!(foo_if_fizz("fuzz"), "bar");
    }

    #[test]
    fn default_to_baz() {
        assert_eq!(foo_if_fizz("literally anything"), "baz");
    }
}

// Task - #3

#[allow(dead_code)]
fn animal_habitat(animal: &str) -> &str {
    let identifier: i32 = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2
    } else if animal == "snake" {
        3
    } else {
        4
    };

    if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    }
}
#[cfg(test)]
mod tests_animal_habitat {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}
