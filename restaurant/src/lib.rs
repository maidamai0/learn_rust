mod front_of_house {
    pub mod hosting {
        pub fn add_to_waiting_list() {}
        fn seat_at_table() {}
    }

    mod serbing {
        fn take_over_table() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        println!("{}", "I'm fixing the order");
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("banana"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    println!("{}", "I'm eating at the front of the house");

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("{}", meal.toast);

    crate::front_of_house::hosting::add_to_waiting_list();
    front_of_house::hosting::add_to_waiting_list();
}
