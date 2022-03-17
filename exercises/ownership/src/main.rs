/** 所有权 */
fn main() {
    let s = String::from("hello");  // s 进入作用域
    let s1 = String::from("world");

    let s2 = takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效
    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，
                                    // 所以在后面可继续使用 x
    let len = ref_func(&s1);        // 对s1引用，不移交所有权

    // println!("s is {}", s); // error: value borrowed here after move
    println!("s is {}", s2); // 所有权移动到 s2 上，访问正常
    println!("x is {}", x); // 可以继续访问
    println!("s1 is {},  it is length is {}", s1, len); // s1可以访问

    let ss = String::from("hello world");
    let hello = slice_func(&ss);
    println!("the first word is: {}", hello); // "hello"

} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 没有特殊之处

fn takes_ownership(some_string: String) -> String{ // some_string 进入作用域
    println!("{}", some_string);
    some_string
} // 这里，some_string 移出作用域并调用 `drop` 方法。
  // 占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。没有特殊之处

fn ref_func(s: &String) -> usize{
    s.len()
}

// slice
fn slice_func(s:&String) -> &str{
    let bytes = s.as_bytes();
    // for (i, &item) in str.iter().enumerate() {
    //     if item == b' ' {
    //         return i;
    //     }
    // }

    // str.len()
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}