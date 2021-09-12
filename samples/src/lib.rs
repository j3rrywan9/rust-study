#![warn(rust_2018_idioms)]
#![allow(unused)]

pub type RGB = (i16, i16, i16);

pub fn color(c: &str) -> RGB {
    (1, 1, 1)
}

pub fn show(c: fn(&str) -> RGB) {
    println!("{:?}", std::mem::size_of_val(&c));
    println!("{:?}", c("black"))
}

pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);

    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }

        m = m % n;
    }

    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}
