fn main() {
    println!("Hello, world!");

    #[derive(Debug)]
    enum IpEnum {
        V4,
        V6
    }

    // #[derive(Debug)]
    // struct IpAddr {
    //     kind : IpEnum,
    //     address: String
    // }

    // let home = IpAddr {
    //     kind: IpEnum::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // println!("home content is {:?}", home);

    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    println!("home content is {:?}", home);
}
