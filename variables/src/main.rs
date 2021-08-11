use std::io;

fn main() {
    let mut x = 5;
    println!("the value of x is {}", x);
    x = 6;
    println!("the value of x is {}", x);
    let x = x * 2;
    println!("the value of x is {}", x);
    let spaces = " \n   ";
    let spaces = spaces.len();
    println!("size of spaces is {}", spaces);
    let guess: u32 = "42".parse().expect("Not a number");
    println!("guess is {}", guess);

    let tup: (i32, f64, u8) = (1, 2.0, 3);
    let (a, b, c) = tup;
    println!("tup is ({} {} {})", tup.0, tup.1, tup.2);
    println!("tup is ({} {} {})", a, b, c);

    let a = [1,2,3,4,5,6];
    println!("please enter a array index");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("failed to read line");
    let index: usize = index.trim().parse().expect("Not a number");
    println!("the value of index is {}",a[index]);
}
