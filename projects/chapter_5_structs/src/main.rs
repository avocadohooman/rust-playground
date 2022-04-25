
struct User { 
	active: bool,
	username: String,
	email: String,
	sign_in_count: u64,
}

fn main() {
    let mut user1 = build_user(String::from("gerhard@minimumbadas.com"), String::from("gmolin"));

	let user2 = User {
		email: String::from("dropaline@gerhardmolin.com"),
		..user1 // spread similar to typescript
	};
}

fn build_user(email: String, username: String) -> User {
	return User {
		email,
		username,
		active: true,
		sign_in_count: 1,

	}
}