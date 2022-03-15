
fn printMyStr(){
    println!("next peint!")
}
// 每个文件必须有个main函数
// fuction 定义语法： fn xxx(){}
fn main(){
    // println!("hello world /n next line"); // print-line
    println(printMyStr())
}

// 首先 Rust 的缩进风格使用 4 个空格，而不是 1 个制表符（tab）。
// 第二，println! 调用了一个 Rust 宏（macro）。如果是调用函数，则应输入 println（没有!）。
// 第三，"Hello, world!" 是一个字符串。我们把这个字符串作为一个参数传递给 println!
// 第四，该行以分号结尾（;），这代表一个表达式的结束和下一个表达式的开始。