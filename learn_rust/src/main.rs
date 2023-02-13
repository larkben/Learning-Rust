struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    is_online: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("spartinofarrows@gmail.com"),
        username: String::from("benjamin"),
        is_online: true,
        sign_in_count: 1
    };

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
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        is_online: true,
        sign_in_count: 1,
    }
} 