use std::io;

fn main() {
    println!("How many Fahrenheit would you like to convert to Celsius?");

	loop {
		let mut fahrenheit = String::new();

		io::stdin()
			.read_line(&mut fahrenheit)
			.expect("Failed to read line");
		
		let fahrenheit: f64 = match fahrenheit.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Invalid number");
				continue;
			}
		};
		println!("{:.1} fahrenheit are {:.2} celsius!", fahrenheit, convert_to_celsius(fahrenheit));
		break;
	}
}

fn convert_to_celsius(fahrenheit: f64) -> f64 {
	let celsius: f64 = (5.0 * (fahrenheit - 32.0)) / 9.0;
	celsius 
}