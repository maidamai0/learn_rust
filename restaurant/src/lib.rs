mod front_of_house;

pub use crate::front_of_house::hosting;

fn serve_order() {}

mod back_of_house {
   pub enum Appetizer {
        Soup,
        Salad,
    }
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

use std::collections::HashMap;
use std::io::Result as ioResult;
use std::fmt::Result;

fn use_hash_map()  {
    let mut map = HashMap::new();
    map.insert("key", Ok("value"));
    map.insert("key2", Err("error"));
}


pub fn eat_at_restaurant() {
    println!("{}", "I'm eating at the front of the house");

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("{}", meal.toast);

    hosting::add_to_waiting_list();
    hosting::add_to_waiting_list();

    let order = back_of_house::Appetizer::Soup;
    let order_1 = back_of_house::Appetizer::Salad;
}
