fn main() {
    let mut user1 = User {
        email: String::from("tom@qq.com"),
        active: true,
        username: String::from("uomi"),
        sign_in_count: 1,
    };

    user1.email = String::from("123@qq.com");

    let user2 = User {
        email: String::from("aopligei@qq.com"),
        ..user1
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email,
        sign_in_count: 2,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
