fn main() {
    println!("Hello, world!");
    let mut s = String::from("Hello, world!");
    s.push_str("fsfs");
    let s1 = &mut s;
    modify(s1);
}

fn modify(str: &mut String) {
    str.push_str("!");
    println!("modifying {}", str);
}
