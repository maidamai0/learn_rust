fn main() {
    let rec = Rectangle {
        width: 10,
        height: 20,
    };
    println!("{:?}", rec);
    println!("{}", rec.area());

    let rec2 = Rectangle {
        width: 9,
        height: 19,
    };

    let rec3 = Rectangle {
        width: 11,
        height: 21,
    };
    println!(
        "rectangle {:?} can hold {:?}:{}",
        rec,
        rec2,
        rec.can_hold(&rec2)
    );
    println!(
        "rectangle {:?} can hold {:?}:{}",
        rec,
        rec3,
        rec.can_hold(&rec3)
    );

    let rec4 = Rectangle::square(10);
    println!("{:?}", rec4);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width <= self.width && other.height <= self.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
