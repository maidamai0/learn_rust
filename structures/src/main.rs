fn main() {
    let user = User {
        username: String::from("joe"),
        email: String::from("123@456.com"),
        sign_in_count: 10,
        active: true,
    };
    print_user(&user);

    let user2 = create_user("1234", "fsajfas@fkdsaljf.com");
    print_user(&user2);


}

fn print_user(user: &User) {
    println!("{} {} {} {}", user.username, user.email, user.sign_in_count, user.active);
}

fn create_user(username: &str, email: &str) -> User{
    User {
        username: String::from(username),
        email: String::from(email),
        sign_in_count:0,
        active:true,
    }
}

struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}
