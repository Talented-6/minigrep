use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    // println!("with text:\n{}", contents);

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("Has found in line {}: {}", line.1, line.0);
    }
    Ok(())
}

// default search method
fn search<'a>(query: &str, contents: &'a str) -> Vec<(&'a str, i32)> {
    // zero-cost abstraction
    contents.lines().zip(1..).filter(|(x, _)| {
        x.contains(query)
    }).collect()

    // to be improved
    // let mut i = 0;
    // contents.lines().filter(|x| x.contains(query)).map(|x| {
    //     i += 1;
    //     (x, i)
    // }).collect()

    // let mut results = Vec::new();
    // let mut i = 0;
    // for line in contents.lines() {
    //     i += 1;
    //     if line.contains(query) {
    //         results.push((line, i));
    //     }
    // }
    // results
}

// case_insensitive search method
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<(&'a str, i32)> {
    contents.lines().zip(1..).filter(|(x, _)| {
        x.to_lowercase().contains(&query.to_lowercase())
    }).collect()

    // contents.lines().enumerate().filter(|(_, x)| {
    //     x.to_lowercase().contains(&query.to_lowercase())
    // }).map(|(i , x)|
    //     (x , i as i32 + 1)
    // ).collect()

    // let mut results = Vec::new();
    // let query = query.to_lowercase();
    // let mut i = 0;
    // for line in contents.lines() {
    //     i += 1;
    //     if line.to_lowercase().contains(&query) {
    //         results.push((line, i));
    //     }
    // }
    // results

    // let mut i = 0;
    // contents.lines().filter_map(|x| {
    //     i += 1;
    //     if x.to_lowercase().contains(&query.to_lowercase()) {
    //         Some((x, i))
    //     } else {
    //         None
    //     }
    // }).collect()
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
    pub case_skip_space: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string!"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name!")
        };

        // match the environment variables
        let case_sensitive = match env::var("CASE_INSENSITIVE") {
            Ok(string) => {
                string == "0" 
            }
            Err(_) => true,
        };
        let case_skip_space = match env::var("CASE_SKIP_SPACE") {
            Ok(string) => {
                string != "0"
            }
            Err(_) => false,
        };
        Ok(Config {
            query,
            filename,
            case_sensitive,
            case_skip_space,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "
Rust: 
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!("safe, fast, productive.", search(query, contents)[0].0)
    }

    #[test]
    fn case_insensitive() {
        let query = "RuSt";
        let contents = "\
Rust:
safe, fast, productive
Trust me.";
        assert_eq!(
            vec![("Rust:", 1), ("Trust me.", 3)],
            search_case_insensitive(query, contents),
        );
    }
}
