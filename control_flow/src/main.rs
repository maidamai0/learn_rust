fn main() {
    let number = 5;
    if number > 5 {
        println!("number is greater than 5");
    } else if number < 5 {
        println!("number is less than 5");
    } else {
        println!("number is equal to 5");
    }

    let number_1 = 6;
    let string = if number_1 == 6 {
        "number is 6"
    } else {
        "number is not 6"
    };
    println!("{}", string);

    let mut cnt = 10;
    loop {
        println!("loop {}", cnt);
        cnt -= 1;
        if cnt == 0 {
            break;
        }
    }

    let mut cnt_1 = 10;
    while cnt_1 > 0 {
        println!("while {}", cnt_1);
        cnt_1 = cnt_1 - 1;
    }

    let a = [1, 2, 3, 4, 5];
    for i in a.iter() {
        println!("{}", i);
    }

    for n in 1..4 {
        println!("{}", n);
    }
}
