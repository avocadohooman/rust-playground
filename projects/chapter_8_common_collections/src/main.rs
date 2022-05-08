fn main() {
	let v = vec![1, 2, 3, 4];

	let third: &i32 = &v[2];
	println!("This third element is {}", third);


	let does_not_exist = v.get(100); // using get is always safer when trying to access an index e.g &v[100] (won't compile)
	println!("There is no third element {:?}", does_not_exist);

	match v.get(2) {
		Some(third) => 	println!("This third element is {}", third),
		None => println!("There is no third element"),
	}
	let mut v2 = vec![1, 2, 3, 4];

	for i in &v {
		println!("{}", i);
	}

	for i in &mut v2 {
		*i += 50;
		println!("{}", i);
	}
}
