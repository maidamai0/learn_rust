fn main() {
    let width = 30;
    let height = 50;
    println!(
        "The area of rectangle is {} square pixels",
        area(width, height)
    );

    let rectangle = (30, 50);
    println!(
        "The area of rectangle is {} square pixels",
        area_v2(rectangle)
    );

    let rectangle_v3 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The rectangle is {:#?}", rectangle_v3);
    println!(
        "The area of rectangle is {} square pixels",
        area_v3(&rectangle_v3)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_v2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_v3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
