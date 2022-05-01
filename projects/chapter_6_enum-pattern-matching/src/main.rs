
#[derive(Debug)]
enum IpAddrKind {
	V4(u8, u8, u8, u8),
	V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
enum UsState {
	Alabama, 
	Alaska,
	California,
}
enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter(UsState),
}

impl Message {
	fn call(&self) {
		println!("{:?}", &self);
	}
}

fn main() {
	let _four = IpAddrKind::V4;
	let _six = IpAddrKind::V6;

	let home = IpAddrKind::V4(127, 0, 0, 1);
	let _loopback = IpAddrKind::V6(String::from("::1"));

	let m = Message::Write(String::from("Hello!"));

	m.call();
    println!("home {:?}", home);

	println!("Coin worth {}", value_in_cents(Coin::Quarter(UsState::California)));

	let five = Some(5);
	let six = plus_one(five);
	let none = plus_one(None);

	let dice_role = 9;

	match dice_role {
		3 => println!("fancy hat"),
		7 => println!("remove fancy hat"),
		_ => reroll(),
	}

	let config_max = Some(3u8);
	if let Some(max) = config_max {
		println!("The maximum is configured to be {}", max);
	}
}

fn value_in_cents(coin:Coin) -> u8 {
	match coin {
		Coin::Penny => 1,
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25		
		},
	}
}

fn plus_one(x: Option<i32>) -> Option<i32> {
	match x {
		None => None,
		Some(i) => Some(i + 1),
	}
}

fn reroll() {}