struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = build_user(String::from("someone@example.com"), String::from("someusername123"));
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("another"),
        ..user1
    };
    println!("User1: {}", user1.username);
    println!("User2: {}", user2.username);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Black: {}, {}, {}", black.0, black.1, black.2);
    println!("Origin: {}, {}, {}", origin.0, origin.1, origin.2);
}
