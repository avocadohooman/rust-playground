use std::collections::HashMap;

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

	let data = "initial contents";

	let mut s = data.to_string(); // same as let s = String::new("initial contents")
	println!("{}", s);
	s.push_str("bar");
	println!("{}", s);
	s.push('!');
	println!("{}", s);

	let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s4 = s1 + "-" + &s2 + "-" + &s3;

	let hello = "Здравствуйте";
	let part_hello = &hello[0..4]; // results in Зд as each character has 2 bytes (not 1 byte, because of of encoding to UTF-8)

	for c in hello.chars() {
		println!("{}", c);
	}

	for c in hello.bytes() {
		println!("{}", c);
	}

	let mut scores = HashMap::new();

	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 20);
    let team_name = String::from("Blue");
	let score = scores.get(&team_name);

	for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

	// inserts only if key doesn't have a value
	scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
}
