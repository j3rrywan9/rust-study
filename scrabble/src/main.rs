use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

const WORDS_PATH: &str = "/usr/share/dict/words";

fn main() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    if let Ok(lines) = read_lines(WORDS_PATH) {
        for line in lines {
            if let Ok(word) = line {
                let mut char_vec = word.chars().collect::<Vec<_>>();
                char_vec.sort();

                let key = String::from_iter(char_vec);
                if !map.contains_key(&key) {
                    map.insert(key.clone(), Vec::new());
                    map.get_mut(&key).unwrap().push(word);
                } else {
                    map.get_mut(&key).unwrap().push(word);
                }
            }
        }
    }

    println!("{:?}", map.get("acer").unwrap());
}

fn read_lines<P>(path: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}
