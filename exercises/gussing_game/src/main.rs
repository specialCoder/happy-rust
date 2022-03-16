use std::io;
use rand::Rng;

fn main() {
    println!("let us guss a number");
    let mut guss_num = String::new();

    let secret = rand::thread_rng().gen_range(1..101);
    println!("secret number is {}", secret);

    println!("guss start ~");
    println!("input your number");
    io::stdin().read_line(&mut guss_num).expect("Failed");

    println!("you guss: {}", guss_num);
    // if guss_num < secret // error 类型不一致无法比较
}
