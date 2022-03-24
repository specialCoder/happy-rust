fn main() {
    println!("Hello, world!");
    // let v = vec![1,2,3];
    // let vs: Vec<_> = v.iter().map(|x| {
    //     return x * x;
    // }).collect();

    // println!("vs is {:?}", vs);
    // println!("vs is {:?}", v)
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);
    println!("in_my_size is {:?}", &in_my_size);

}

#[derive(Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|shoe| {
        shoe.size == size
    }).collect()
}

struct Counter{
    count: u32
}

impl Counter{
    fn new(count:u32) -> Counter{
        Counter{
            count,
        }
    }
}

impl Iterator for Counter{
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item>{
        if self.count < 5{
            self.count += 1;
            Some(self.count)
        }else{
            None // 返回Node 表示迭代器迭代结束
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // #[test]
    // fn iterator_demonstration() {
    //     let v1 = vec![1, 2, 3];
    
    //     let mut v1_iter = v1.iter();

    //     // 调用 sum 之后不再允许使用 v1_iter 因为调用 sum 时它会获取迭代器的所有权。
    //     // let total: i32 = v1_iter.sum();
    
    //     assert_eq!(v1_iter.next(), Some(&1));
    //     assert_eq!(v1_iter.next(), Some(&2));
    //     assert_eq!(v1_iter.next(), Some(&3));
    //     assert_eq!(v1_iter.next(), None);
    // }

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new(0);

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        // assert_eq!(counter.next(), Some(6));
        assert_eq!(counter.next(), None);
    }
}
