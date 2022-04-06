fn main() {
    println!("Hello, world!");
    test_array();
}

// array
fn test_array(){
    let arr_1:[i32;3] = [1,2,3];
    println!("arr_1 index 0 is {}", arr_1[0]);

    let arr_2 = [0;5];
    println!("arr_2 is {:?}, length is {}", arr_2, arr_2.len());
    let len = arr_2.len();
}