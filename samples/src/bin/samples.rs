use samples::{color, show};

fn main() {
    let rgb = color;
    show(rgb);

    println!("Size of &rgb: {:?}", std::mem::size_of_val(&rgb));

    let c = |_: &str| { (1, 2, 3) };
    show(c);
}
