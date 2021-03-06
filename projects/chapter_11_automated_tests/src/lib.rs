#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub struct Guess {
    value: i32,
}

impl Guess {
	pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }
		Guess {value}
	}
}

#[cfg(test)]
mod tests {
	/*
		Note that we’ve added a new line inside the tests module: use super::*;. 
		The tests module is a regular module that follows the usual visibility rules 
		we covered in Chapter 7 in the “Paths for Referring to an Item in 
		the Module Tree” section. Because the tests module is an inner module,
		we need to bring the code under test in the outer module into the scope 
		of the inner module. We use a glob here so anything we define in the outer 
		module is available to this tests module.
	*/
	use super::*;

	#[test]
	fn it_works() -> Result<(), String> {
		if 2 + 2 == 4 {
			Ok(())
		} else {
			Err(String::from("two plus two does not equal four"))
		}
	}

    #[test]
	#[ignore]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

	#[test]
	fn larger_can_hold_smaller() {
		let larger = Rectangle {
			width: 15,
			height: 10,
		};

		let smaller = Rectangle {
			width: 10,
			height: 5
		};

		assert!(larger.can_hold(&smaller));
	}

	#[test]
	fn smaller_cannot_hold_larger() {
		let larger = Rectangle {
			width: 15,
			height: 10,
		};

		let smaller = Rectangle {
			width: 10,
			height: 5
		};

		assert!(
			!smaller.can_hold(&larger),
			"Smaller could hold larger, value was `{:?}`",
			smaller,
		);
	}

	#[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

}
