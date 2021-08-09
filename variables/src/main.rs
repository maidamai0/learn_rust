fn main() {
    let mut x=5;
    println!("the value of x is {}", x);
    x = 6;
    println!("the value of x is {}", x);
    let x = x * 2;
    println!("the value of x is {}", x);
    let spaces = " \n   ";
    let spaces = spaces.len();
    println!("size of spaces is {}", spaces);
    let guess: u32 = "42".parse().expect("Not a number");
    println!("guess is {}", guess)
}