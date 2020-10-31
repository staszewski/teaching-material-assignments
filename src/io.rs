use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use url::Url;

pub fn io() {
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
                    match Url::parse(&url) {
                        Ok(u) => {
                            println!("{:?}", Some(&u));
                            Some(&u)
                        }
                        Err(_) => None,
                    };
                };
            }
            Err(error) => panic!("couldnt {}", error),
        }
    }
}
