fn main() {
    let mut s = String::from("Hello");
    s.push_str(", world");
    println!("{}", s);

    let mut s2 = String::from("Hello");
    {
        let s3 = String::from("world");
        s2 = s3.clone();
        println!("s2 is {}", s2);
        println!("s3 is {}", s3);
    }

    take_ownership(s2);
    let x = 3;
    make_copy(x);
}

fn take_ownership(s: String) {
    println!("{}", s);
}

fn make_copy(i: i32) {
    println!("{}", i);
}
