//! Implement `StrSplit` between <https://www.youtube.com/watch?v=rAl-9HwD858&t=216s> and <https://www.youtube.com/watch?v=rAl-9HwD858&t=4524s>

/// `StrSplit` lets you take a string and split it by the string and walk the splits of that string
#[derive(Debug)]
pub struct StrSplit<'haystack, 'delimiter> {
    remainder: Option<&'haystack str>,
    delimiter: &'delimiter str,
}

impl<'haystack, 'delimiter> StrSplit<'haystack, 'delimiter> {
    pub fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

/// Alternatively, with the placeholder lifetime
/// ```rust
/// impl<'haystack> Iterator for StrSplit<'haystack, '_> {}
/// ```
impl<'haystack, 'delimiter> Iterator for StrSplit<'haystack, 'delimiter> {
    type Item = &'haystack str;

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;

        if let Some(next_delim) = remainder.find(self.delimiter) {
            let until_delimiter = &remainder[..next_delim];
            *remainder = &remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else {
            self.remainder.take()
        }
    }
}

/// With explicit lifetime
/// ```rust
/// fn until_chars<'s>(s: &'s str, c: char) -> &'s str {}
/// ```
fn until_chars(s: &str, c: char) -> &'_ str {
    let delim = format!("{}", c);

    StrSplit::new(s, &delim)
        .next()
        .expect("StrSplit always gives at least one result")
}

#[test]
fn until_char_test() {
    assert_eq!(until_chars("hello world", 'o'), "hell");
}

#[test]
fn it_works() {
    let haystack = "a b c d";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d"]);
}

#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}
