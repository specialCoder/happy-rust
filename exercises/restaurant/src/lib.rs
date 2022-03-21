mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // let result = String::from("28");

    // hosting::add_to_waitlist(&result);

    hosting::add_to_waitlist();

    // 绝对路径
    // crate::front_of_house::hosting::add_to_waitlist(&result);

    // 相对路径
    // front_of_house 还不是公有的，会报错
    // front_of_house::hosting::add_to_waitlist();
}
