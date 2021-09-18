
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    // mod serving {
    //     fn take_order() {}

    //     fn serve_order() {}

    //     fn take_payment() {}
    // }
}

mod back_of_house {
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn serve_order() {}

    fn fix_incorrect_order() {

    }
}

use crate::front_of_house::hosting; // shortcut for line 48

pub fn eat_at_restaurant() {
    // absolute path
    hosting::add_to_waitlist();

    // relative path
    hosting::add_to_waitlist();

    let order_1 = back_of_house::Appetizer::Soup;
    println!("{:#?}", order_1);

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}



