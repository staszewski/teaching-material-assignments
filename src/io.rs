use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use url::Url;

pub fn parse_line(line: String) -> Option<Url> {
    match Url::parse(&line) {
        Ok(u) => Some(u),
        Err(e) => None,
    }
}

fn io() {
    let path = Path::new("src/data/content.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let lines = BufReader::new(file);

    for line in lines.lines() {
        match line {
            Ok(url) => {
                if !url.is_empty() {
                    parse_line(url)
                } else {
                    None
                };
            }
            Err(error) => panic!("couldnt {}", error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_correctly() {
        assert!(parse_line(String::from("https://example.com")).is_some());
    }

    #[test]
    fn no_url() {
        assert!(parse_line(String::from("abcdf")).is_none())
    }
}
