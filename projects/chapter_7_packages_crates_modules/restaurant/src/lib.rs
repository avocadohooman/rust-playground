/*
	use std::{cmp::Ordering, io};
	same as
	use std::cmp::Ordering;
	use std::io;
*/

/*
	same as
	use std::io;
	use std::io::Write;
*/
use std::io::{self, Write};

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

// this brings Hosting into scope for the eat_at_restaurant fn
// when applying use, it is a recommended convention to bring the closest parent
// into scope, instead of a particular child
// pub use makes this mod also avaialble to external code
pub use crate::front_of_house::hosting as Hosting;

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
	Hosting::add_to_waitlist();
}
