mod front_of_house {
	pub mod hosting {
		pub fn add_to_waitlist() {}

		fn seat_at_table() {}
	}

	mod service {
		fn take_oder() {}

		fn serve_order() {}

		fn take_payments() {}
 	}
}

fn serve_order() {}

mod back_of_house {
	pub struct Breakfast {
		pub toast: String,
		season_fruit: String,
	}

	pub enum Appetize {
		Soup,
		Salad,
	}

	impl Breakfast {
		pub fn summer(toast: &str) -> Breakfast {
			Breakfast {
				toast: String::from(toast),
				season_fruit: String::from("peaches"),
			}
		}
	}

	fn fix_incorrect_order() {
		cood_order();
		super::serve_order();
	}

	fn cood_order() {}
}

// this allows to simply call hosting:: for the eat_at_restaurant fn
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
	// Order a breakfast in the sumner with Rye toast
	let mut meal = back_of_house::Breakfast::summer("Rye");
	// Change our mind about what bread we'd like
	meal.toast = String::from("Wheat");
	let order_1 = back_of_house::Appetize::Salad;
	println!("I'd like {} toast please", meal.toast);
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
	hosting::add_to_waitlist();
}
