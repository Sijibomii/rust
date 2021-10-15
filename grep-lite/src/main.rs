use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use clap::{App,Arg};

    fn main() {
        let args = App::new("grep-lite")
            .version("0.1")
            .about("searches for patterns")
            .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
            .arg(Arg::with_name("input")
            .help("File to search")
            .takes_value(true)
            .required(true))
            .get_matches();
        let pattern = args.value_of("pattern").unwrap();
        let re = Regex::new(pattern).unwrap();
        let input = args.value_of("input").unwrap();
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        for line_ in reader.lines() {
            let line = line_.unwrap();
            match re.find(&line) {
                Some(_) => println!("{}", line),
                None => (),
                }
            }
        // let quote = "Every face, every shop, bedroom window, public-house, and
        // dark square is a picture feverishly turned--in search of what?
        // It is the same with books. What do we seek through millions of pages?";
        // for line in quote.lines() {
        //     let contains_substring = re.find(line);
        //     match contains_substring {
        //     Some(_) => println!("{}", line),//re.find() returns either a some when theres a match or a none when thered none
        //     None => (),
        //     }
        // }
    }