fn main() {
    println!("Hello, world!");
    let tup:(i32,f64, u8) = (40,120.5, 12);
    let (x, y, _z) = tup;
    println!("x is {}, y is {}, z is {}", x, y, tup.2);
}
