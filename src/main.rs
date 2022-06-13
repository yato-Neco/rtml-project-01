mod lib;
use lib::parsing;
use std::io::prelude::*;
use std::fs::File;
fn main() {


    let mut f = File::open("index.rtml").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut t = parsing::Lexer::new(&contents);


    let p = parsing::Lexer::convert(&mut t);

    for i in p {
        println!("{:?}",i);
    }

}
