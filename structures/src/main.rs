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


    let user3 = User {
        username: String::from("copy"),
        email:String::from("copy@123.com"),
        ..user2
    };
    print_user(&user3);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    println!("black is [{}, {}, {}]", black.0, black.1, black.2);
    println!("origin is [{}, {}, {}]", origin.0, origin.1, origin.2);

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

