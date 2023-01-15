use std::{fs, error::Error, env};
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3{
            println!("Not enough arguments")
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
        Ok(Config { query, filename, case_sensitive})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let file = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &file)
    } else {
        search_case_insensitive(&config.query, &file)
    };

    for line in results{
        println!("{}", line);
    }

    Ok(())

}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines(){
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
        let query =  "lorem";
        let contents=  "Some text which 
includes Lorem ipsum
an so on";

        assert_eq!(vec!["includes lorem ipsum"], search(query, contents));
    }

    #[test]
    fn case_insensitive(){
        let query =  "LoReM";
        let contents=  "
Some
text which 
includes lorem ipsum
and so on";

        assert_eq!(vec!["lorem", "and so on"], search_case_insensitive(query, contents));
    }
}
