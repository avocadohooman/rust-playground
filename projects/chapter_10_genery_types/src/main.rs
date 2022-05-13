struct Point<T> {
	x: T,
	y: T,
}

struct MixedPoint<T, U> {
	x: T,
	y: U,
}

fn main() {
	// when using generic type for structs, the passed values need to have the same type
	let integer = Point { x: 5, y: 10};
	let float = Point { x: 1.0, y: 4.0};

	let integer_float = MixedPoint { x: 5, y: 1.0 };

	let string1 = String::from("long string is long");
	let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

fn longest<'la>(x: &'la str, y: &'la str) -> &'la str {
	if x.len() > y.len() {
		x
	} else {
		y
	}
}