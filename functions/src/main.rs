fn main() {
    println!("Hello, world!");
    another_function();
    ana_function_with_parameter(89);
    block_as_expression();
    println!("function_return_values() return {}", function_return_values());
}

fn another_function() {
    println!("another_functions");
}

fn ana_function_with_parameter(x: i32) {
    println!("x is {}", x);
    let y = {
        let x = 4;
        x + 1
    };

    println!("{}", y);
}

fn block_as_expression() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn function_return_values() -> i32{
    return 5;
}
