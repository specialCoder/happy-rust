fn main() {
    println!("Hello, world!");
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];

    let mut numbers: [u32; 3] = [1, 2, 3];
    // println!("v is {:?}", v)

    let new_num = add(&mut numbers);

    // println!("new value is {:?}", numbers);
    println!("new_number is {:?}", new_num);

    let mut s = String::from("hello");

    s.push_str(" world");

    let ss = &s[0..5];
    println!("s is {}; ss is {}", s, ss);

    // let abc = s + "s"; // ok

    for c in "hello".chars() {
        println!("{}", c);
    }

    for b in "hello".chars() {
        println!("{}", b);
    }
}

fn add(add: &mut [u32; 3]) -> &[u32; 3] {
    add[0] = 2;
    return add;
}
