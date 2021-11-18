// import standard input/output module
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess th number!");

    // 使用的范围表达式采用 start..end 形式
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Teh secret number is {}", secret_number);
    
    loop{
        println!("Please input you guess.");

        // let 创建一个变量，默认是immutable
        // mut（multiple）: 使一个变量可变
        // String::new 这个函数会返回一个 String 的新实例。
        // :: 表示关联性
        let mut guess = String::new();
    
        // 调用 io 库中的函数 stdin
        // & 表示这个参数是一个 引用，它允许多处代码访问同一处数据
        // expect: 我们没有使用 read_line 的返回值 Result，说明有一个可能的错误没有处理。
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
    
        // {} 是预留在特定位置的占位符
        println!("you guessed: {}", guess);
    
        // Rust 允许用一个新值来 隐藏 （shadow） guess 之前的值
        // let guess: u32。guess 后面的冒号（:）告诉 Rust 我们指定了变量的类型。
        // 字符串parse方法 将字符串解析成数字
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
    
        match guess.cmp(&secret_number){
            // Ordering 也是一个枚举，不过它的成员是 Less、Greater 和 Equal
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
