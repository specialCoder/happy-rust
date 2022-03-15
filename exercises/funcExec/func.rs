
// 基础版
// fn main() {
//     println!("Hello, world!");
//     another_function();
// }

// fn another_function() {
//     println!("Hello, runoob!");
// }

// 带参数和返回值版
fn main() {
    let result = another_function(5, 6);
    println!("x + y = {}", result)
}

fn another_function(x: i32, y: i32)-> i32 {
    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);
    return x + y;
}
