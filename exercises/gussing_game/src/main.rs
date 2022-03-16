use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("let us guss a number");
    // 生成一个随机数
    let secret:u32 = rand::thread_rng().gen_range(1..101);
    println!("secret number is {}", secret);
    println!("guss start ~");

    loop { // 开始循环猜测
      // 初始化一个输入值为空字符串
      let mut guess_num = String::new();
      println!("input your number");
      // 接收输入的字符串
      io::stdin().read_line(&mut guess_num).expect("Failed");
  
      //   let guess_num:u32 = guess_num.trim().parse().expect("Please type a number!");
      // 使用 shadowing 重新赋值： 把输入的字符串转为 u32 整数
      // 字符串的trim() 
      // parse() 方法 返回一个 structure
      // 使用 match 得到匹配的数字
      let guess_num:u32 = match guess_num.trim().parse() {
          Ok(num) => num, // ok的处理
          Err(_) => continue//error 的处理
      };

      println!("you guss: {}", guess_num);
      // if guess_num < secret // error 类型不一致无法比较
  
      // 进行比较
      // switch enum
      match guess_num.cmp(&secret) {
          Ordering::Less => println!("Too smalll"),
          Ordering::Greater => println!("Too big"),
          Ordering::Equal => {
              println!("You win");
              break; //  break loop ：正确之后退出循环
          }
      }
  }
}
