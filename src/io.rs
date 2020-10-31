use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::Path;

pub fn io() {
    let path = Path::new("src/data/content.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut reader = BufReader::new(file);
    let mut s = String::new();

    /* match reader.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!("{}", s),
    } */

    for read in reader.lines() {
        println!("line {}", read.unwrap());
    }
}
