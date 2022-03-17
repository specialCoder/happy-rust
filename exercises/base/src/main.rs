use std::io;

fn main() {
    println!("entry main ~~~");
    // test_array();
    // let r = has_return_fn(3);
    // println!("result is {}", r)
    get_resut(true)
    
}

// return function
fn has_return_fn(x: u32) -> u32{
    let y = {
       let z = x + 1;
       z * z
    };

    x + y
}

// let if
fn get_resut(condition: bool){
    let y = if condition {
        "right"
    } else {
        "error"
    };

    // match y { // error
    //     "right" => 100,
    //     "error" => 0
    // }
    println!("resut is {}", y);
}

// array demo
fn test_array() -> (){
    let a = [1,2,3,4];
    let mut input = String::new();
    println!("input array index");

    io::stdin().read_line(&mut input).expect("input failed");
    let input_num: usize = input.trim().parse().expect("parsefFailed");
    println!(" current index is {index}, array value is {value}", index=input_num, value=a[input_num])
}
