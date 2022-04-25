
struct User { 
	active: bool,
	username: String,
	email: String,
	sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let mut user1 = build_user(String::from("gerhard@minimumbadas.com"), String::from("gmolin"));

	let user2 = User {
		email: String::from("dropaline@gerhardmolin.com"),
		..user1 // spread similar to typescript
	};

	let black = Color(0, 0, 0);
	let origin = Point(0, 0, 0);

	let subject = AlwaysEqual;


}

fn build_user(email: String, username: String) -> User {
	return User {
		email,
		username,
		active: true,
		sign_in_count: 1,

	}
}