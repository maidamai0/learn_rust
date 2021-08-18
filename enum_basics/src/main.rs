fn main() {
    let ip_v4 = IpAddressKind::V4(127, 0, 0, 1);
    let ip_v6 = IpAddressKind::V6(String::from("2001:db8::1"));
    println!("ip_v4 is {:?}", ip_v4);
    println!("ip_v6 is {:?}", ip_v6);

    // let ip_v4_addr = IpAddress::new(ip_v4, "127.0.0.1");
    // let ip_v6_addr = IpAddress::new(ip_v6, "::1");
    // println!("ip_v4_addr is {:?}", ip_v4_addr);
    // println!("ip_v6_addr is {:?}", ip_v6_addr);

    let quit_msg = Message::Quit;
    let move_msg = Message::Move{x:1, y:2};
    let write_msg = Message::Write{text: String::from("Hello, world!")};
    let change_color_msg = Message::ChangeColor(125,125,125);
    println!("quit_msg is {:?}", quit_msg);
    quit_msg.call();
    println!("move_msg is {:?}", move_msg);
    move_msg.call();
    println!("write_msg is {:?}", write_msg);
    write_msg.call();
    println!("change_color_msg is {:?}", change_color_msg);
    change_color_msg.call();
}

#[derive(Debug)]
enum IpAddressKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

// #[derive(Debug)]
// struct IpAddress {
//     kind: IpAddressKind,
//     address: String
// }

// impl IpAddress {
//     fn new(kind: IpAddressKind, address: &str) -> IpAddress {
//         IpAddress {
//             kind,
//             address: String::from(address)
//         }
//     }
// }

#[derive(Debug)]
enum Message{
    Quit,
    Move{x:i32, y:i32},
    Write{text: String},
    ChangeColor(u8, u8, u8),
}

impl Message {
    fn call(self) {
        println!("call {:?}", self);
    }
}