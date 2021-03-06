use std::env;
use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive{
      search(&config.query, &content);
    }else{
      search_case_insensitive(&config.query, &content);
    };

    for line in results{
      println!("{}", line);
    }
    Ok(())
}
pub struct Config{
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Config{
    pub fn new(args: &[String]) -> Result< Config, &str>{
        if args.len() < 3 {
            return Err("not enough args");
        }
        let query = args[1].clone();
        let filename  = args[2].clone();
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();
       Ok(Config { query, filename, case_sensitive })
    }
}
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str>{
  let mut result = Vec::new();
  for line in content.lines() {
    if line.contains(query){
      result.push(line);
    }
  }
  result
}
pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str>{
  let query = query.to_lowercase();
  let mut results = Vec::new();
  for line in content.lines() {
    if line.to_lowercase().contains(&query){
      results.push(line);
    }
  }
  results
}

#[cfg(test)]
mod tests{
  use super::*;

  #[test]
  fn case_sensitive(){
    let query = "duct";
    let content = "safe fast productive";
    assert_eq!(vec!["safe fast productive"], search(query, content));
  }

  #[test]
  fn case_insensitive(){
    let query = "rUsT";
    let content = "Rust, baby";
    assert_eq!(vec!["Rust, baby"], search_case_insensitive(query, content));
  }
}