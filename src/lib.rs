use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    /// Returns a `Result` value that will contain a `Config` instance in the
    /// successful case and will describe the problem in the error case.
    pub fn new(args: &[String]) -> Result<Config, &str> {
        //using panic! is more appropriate for a programming problem than a usage problem
        if args.len() < 3 {
            //panic!("not enough arguments");
            return Err("not enough arguments");
        }

        //the `args` variable is the owner of the argument values and is only
        //letting `parse_config` borrow them, which means we would violate Rust's
        //borrowing rules if `Config` tried to take ownership of the values in `args`.
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {
            query: query,
            filename: filename,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new() {
        let v = vec![];
        assert_eq!("not enough arguments", Config::new(&v).unwrap_err());

        let v = vec!["minigrep".to_string(), "2".to_string()];
        assert_eq!("not enough arguments", Config::new(&v).unwrap_err());

        let v = vec![
            "minigrep".to_string(),
            "word".to_string(),
            "data.txt".to_string(),
        ];
        assert_eq!(
            Config {
                query: v[1].clone(),
                filename: v[2].clone()
            },
            Config::new(&v).unwrap()
        );

        let v = vec![
            "minigrep".to_string(),
            "word".to_string(),
            "data.txt".to_string(),
            "extra-arg".to_string(),
        ];
        assert_eq!(
            Config {
                query: v[1].clone(),
                filename: v[2].clone()
            },
            Config::new(&v).unwrap()
        );
    }
}

//use the trait object `Box<dyn Error>` to give flexibility in returning error values
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //`?` will return error value from current function for caller to handle
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

#[test]
fn test_run() {
    let config = Config {
        query: "the".to_string(),
        filename: "./data/poem.txt".to_string(),
    };

    assert_eq!((), run(config).unwrap());

    let config = Config {
        query: "the".to_string(),
        filename: "./data/unknown-file.txt".to_string(),
    };

    //panics if did not get an error
    run(config).expect_err("accessing an invalid file");
}

//the lifetime parameter `'a` specifies which argument lifetime is connected to
//the lifetime of the return value.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[test]
fn test_one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
}
