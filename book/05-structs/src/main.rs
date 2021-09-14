fn main() {
    println!("Hello, world!");
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool
    }

    let mut user1 = User {
        username: String::from("imnirfn"),
        email: String::from("s3ns3xy@yahoo.com"),
        sign_in_count: 1,
        active: true,
    };

    fn build_user(email: String, username: String) -> User {
        User {
            username,
            email,
            active: true,
            sign_in_count: 1,
        }
    }

    user1.username = String::from("user1");
    println!("User's email is: {}", user1.username);

    let mut user2 = build_user(String::from("user@gmail.com"), String::from("user2"));
    println!("User's email is: {}", user2.username);

    let inactive_user = User {
        active: false,
        ..user2
    };
    println!("Inactive user name: {}", inactive_user.username);
}

