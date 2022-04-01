// 格式化输出： https://rustwiki.org/zh-CN/std/fmt/
fn main() {
    #[derive(Debug)]
    struct Demo{
        name:String
    }

    println!("hello world"); // base
    println!("my name is {}, age is{}, age{1}, job is {engine}", "myname", 18, engine="engine");// index / name
    println!("struc is {:#?}", Demo{ name:String::from("demo-name")}); // format

     // 可以在 `:` 后面指定特殊的格式。(二进制)
     println!("{} of {:b} people know binary, the other half don't", 1, 3);

     // 你可以按指定宽度来右对齐文本。
     // 下面语句输出 "     1"，5 个空格后面连着 1。
     println!("{number:>5}", number=1);
     // 下面语句输出 "     1"，5 个空格后面连着 1。
     println!("{number:>width$}", number=1, width=5);
 
     // 你可以在数字左边补 0。下面语句输出 "000001"。
     println!("{number:>0width$}", number=1, width=6);

    // 创建一个包含单个 `i32` 的结构体（structure）。命名为 `Structure`。
    #[derive(Debug)]
    struct Structure(i32);

    // 但是像结构体这样的自定义类型需要更复杂的方式来处理。
    // 下面语句无法运行。
    println!("This struct `{:?}` won't print...", Structure(3));

    let pi:f32 = 3.141592;
    println!("Pi is roughly {0:.2}", pi);

}
