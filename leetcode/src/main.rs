#![allow(unused)]

fn main() {
    let s = Box::new("hello".to_string());

    println!("{:p}", &s);
    println!("{:p}", s.as_ptr());
}
