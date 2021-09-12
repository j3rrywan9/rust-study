use samples::{color, gcd, show};
use std::env;
use std::fs;
use std::str::FromStr;

fn main() {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";
    println!("Size of output: {:?}", std::mem::size_of_val(output));

    println!("Fetching URL: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting HTML to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown has been saved in {}.", output);

    let rgb = color;
    show(rgb);

    println!("Size of &rgb: {:?}", std::mem::size_of_val(&rgb));

    let c = |_: &str| { (1, 2, 3) };
    show(c);

    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        eprint!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}
