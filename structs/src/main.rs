struct User {
    username: String,
    email: String, 
    sign_in_count: u64,
    active: bool
}

fn main() {
    simple_structs();
    structs_and_imp();
}

fn simple_structs() {
    // Could be mutable or immutable
    let mut user1 = User {
        email: String::from("erick@mail.com"),
        username: String::from("erick123"),
        sign_in_count: 0,
        active: true
    };

    let name = user1.username;
    user1.username = String::from("efrain123");

    let user2 = build_user(
        String::from("efra@mail.com"),
        String::from("someone123"),
    );

    // We can create new instances of a user using existing instances
    let user3 = User {
        email: String::from("a@mail.com"),
        username: String::from("a123"),
        ..user2
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        // shorthand fields, not needed email: email or username: username
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}

fn using_tuples() {
    // Have same content but Color != Point
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
}

struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.width
    }
}

fn structs_and_imp() {
    let rectangle1 = Rectangle { width: 10, height: 20 };
    let rectangle2 = Rectangle { width: 5, height: 10 };
    let rectangle3 = Rectangle { width: 20, height: 40 };
    println!("Rectangle area: {}", rectangle1.area());
    println!("Rectangle1 can hold Rectangle2: {}", rectangle1.can_hold(&rectangle2));
    println!("Rectangle1 can hold Rectangle3: {}", rectangle1.can_hold(&rectangle3));
    let rectangle4 = Rectangle::square(5);
    println!("Square area: {}", rectangle4.area());
}

// Asociated functions
impl Rectangle {
    // As it is asociated function it doesn't need self
    fn square(size: i32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}