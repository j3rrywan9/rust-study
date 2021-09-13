use std::fs;

fn main() {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";

    println!("Fetching URL: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting HTML to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown has been saved in {}.", output);
}
