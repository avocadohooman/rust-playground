fn main() {
    let s1 = String::from("hello"); // s1 scope of main

	takes_onwership(s1); // s1 leaves scope of main, and is no lonver valid here

	let x = 5; // x comes into scope
	makes_copy(x); // a copy of x is sent to makes_copy. og x is still part of main scope

	let s_pointer = String::from(", world!"); // immutable pointer

	let mut s_mut_pointer = String::from("hello");
	
	println!("s_mut_pointer before change {}", s_mut_pointer);
	change(&mut s_mut_pointer);
	println!("s_mut_pointer after change {}", s_mut_pointer);

	let r1  = &s_mut_pointer;
	let r2 = &r1;

	println!("r2=r1=s_mut_pointer {} {}", r2, s_mut_pointer);

	let len = calculate_length(&s_pointer);
	println!("len of s_pointer: {}", len);

} // here x goes out of scope -> drop


fn takes_onwership(some_string: String) {
	println!("some_string comes into scope {}", some_string); // s1 comes into scope of takes_ownership
} // here s1/some_string is out of scope and drop is called

fn makes_copy(some_integer: i32) {
	println!("{}", some_integer); // copy of x
} // here copy of x (some_integer) goes out of scope -> drop

fn calculate_length(s: &String) -> usize {
	return s.len();
}

fn change(some_string: &mut String) {
	some_string.push_str(", world!");
}