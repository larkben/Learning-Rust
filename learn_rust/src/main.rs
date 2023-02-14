/*
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    is_online: bool,
}
*/
#[derive(Debug)] // Basic Implementation of Debug Trait
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect1 = Rectangle {
        width: 20,
        height: 80,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 20,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    println!(
        "Rectangle 1 can {} fit inside Rectangle",
        rect.can_hold(&rect1)
    );
    println!(
        "Rectangle 1 can {} fit inside Rectangle",
        rect.can_hold(&rect2)
    );

    println!("{:#?}", rect);

    /*
    let mut user1 = User {
        email: String::from("spartinofarrows@gmail.com"),
        username: String::from("benjamin"),
        is_online: true,
        sign_in_count: 1
    };

    // Struct Tuples
    //struct Color(i32, i32, i32);

    //struct Point(i32, i32, i32);

    // Editing said Struct
    let name = user1.username;
    user1.username = String::from("Benjamin");

    // Building Struct with Function
    let user2: User = build_user(
        String::from("jerry@aol.com"), 
        String::from("bigjerry")
    );

    // Building Struct off Previous User
    let user3: User = User {
        email: String::from("jamesbond@aol.com"),
        username: String::from("kingjames"),
        ..user2
    };
    */
}

/*
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        is_online: true,
        sign_in_count: 1,
    }
} 
*/