// use std::io;

fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS: {}", THREE_HOURS_IN_SECONDS);

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces: {}", spaces);

    let tup = (500, 6.4, 1);

    println!("tup.0 {}, tup.1 {}, tup.2 {}", tup.0, tup.1, tup.2);

    // println!("Please enter an array index.");

    // let mut index = String::new();

    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");

    // let index: usize = index.trim().parse().expect("Enter a valid number");
    // println!(
    //     "get_element_in_array(index) at index{}: {}",
    //     index,
    //     get_element_in_array(index)
    // );
	testing_loop();
}

// fn get_element_in_array(index: usize) -> i32 {
//     let array_type_int32_with_five_elements: [i32; 5] = [1, 2, 3, 4, 5];

//     let element_in_array = array_type_int32_with_five_elements[index];

//     println!(
//         "The value of the element at index {} is {}",
//         index, element_in_array
//     );

//     element_in_array
// }

fn testing_loop() {
	let mut count = 0;

	'counting_up: loop {
		println!("count = {}", count);
		let mut remaining = 10;

		loop {
			println!("remaining = {}", remaining);
			if remaining == 9 {
				break;
			}
			if count == 2 {
				break 'counting_up;
			}
			remaining -= 1;
		}

		count += 1;
	}
	println!("End count {}", count);

	let mut counter_with_return_value = 0;

	let result = loop {
		counter_with_return_value += 1;

		if counter_with_return_value == 10 {
			break counter_with_return_value * 2;
		}
	};
	println!("The result is {}", result);

	let a = [10, 20, 30, 40, 50];

	for element in a {
		println!("the value is {}", element);
	}
}
