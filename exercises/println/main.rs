fn main() {
    #[derive(Debug)]
    struct Structure(i32);

    //{} 占位符
    // {0} 指定变量位置
    println!("Hello, world! show number one:{} two:{} three:{0}", 1, 2);
    // {nam1} 指定变量名称
    println!(
        "name name1:{name1};name2:{name2}",
        name1 = "name001",
        name2 = "name002"
    );
    // output string：Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );
    // {:?} format stdout
    // {} report error
    println!("Structure is {:?}", Structure(12))
}
