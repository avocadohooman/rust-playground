struct Point<T> {
	x: T,
	y: T,
}

struct MixedPoint<T, U> {
	x: T,
	y: U,
}

use aggregator::{Summary, Tweet};

fn main() {
	// when using generic type for structs, the passed values need to have the same type
	let integer = Point { x: 5, y: 10};
	let float = Point { x: 1.0, y: 4.0};

	let integer_float = MixedPoint { x: 5, y: 1.0 };
}