#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // struct User {
    //     email: String,
    //     username: String,
    //     active: bool,
    //     sign_in_count: u32,
    // }

    // let user1 = User {
    //     email: String::from("someone@example.com"),
    //     username: String::from("someusername123"),
    //     active: true,
    //     sign_in_count: 1,
    // };

    // let user2 = User {
    //     email: String::from("another@example.com"), // 不同的写在上面
    //     ..user1
    // };

    // println!("user1 is {}", user1.email);
    // println!("user2 is {}", user2.email);
    impl Rectangle {
        fn width(&self) -> bool {
            self.width > 0
        }
    }

    let rect_one: (u32, u32) = (12, 10);
    let rect_two = Rectangle {
        width: 11,
        height: 11,
    };
    let rect1 = Rectangle {
        width: 11,
        height: 11,
    };

    println!("area_one result  is {}", area_one(rect_one));
    println!("area_two result  is {}", area_two(rect_two));
    // println!("area is {:#?}", rect1)
    // dbg!(&rect1);
    println!("width > 0 ? {}", rect1.width());
}

fn area_one(dimensions: (u32, u32)) -> u32 {
    return dimensions.0 * dimensions.1;
}

fn area_two(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
