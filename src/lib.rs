use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    /// Returns a `Result` value that will contain a `Config` instance in the
    /// successful case and will describe the problem in the error case (less than
    /// 3 arguments- including exe path).
    pub fn new<T>(mut args: T) -> Result<Config, &'static str>
    where
        T: Iterator<Item = String>,
    {
        //skip exe path
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            //using panic! is more appropriate for a programming problem than a usage problem
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query: query,
            filename: filename,
            case_sensitive: case_sensitive,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new() {
        let v = vec![];
        assert_eq!(
            "Didn't get a query string",
            Config::new(v.into_iter()).unwrap_err()
        );

        let v = vec![String::from("minigrep"), String::from("rusty")];
        assert_eq!(
            "Didn't get a file name",
            Config::new(v.into_iter()).unwrap_err()
        );

        let v = vec![
            String::from("minigrep"),
            String::from("word"),
            String::from("data.txt"),
        ];
        assert_eq!(
            Config {
                query: v[1].clone(),
                filename: v[2].clone(),
                case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
            },
            Config::new(v.into_iter()).unwrap()
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
                filename: v[2].clone(),
                case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
            },
            Config::new(v.into_iter()).unwrap()
        );
    }
}

//use the trait object `Box<dyn Error>` to give flexibility in returning error values
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //`?` will return error value from current function for caller to handle
    let contents = fs::read_to_string(config.filename)?;

    //branch based on case sense flag
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

#[test]
fn test_run() {
    let config = Config {
        query: "the".to_string(),
        filename: "./data/poem.txt".to_string(),
        case_sensitive: false,
    };

    assert_eq!((), run(config).unwrap());

    let config = Config {
        query: "the".to_string(),
        filename: "./data/unknown-file.txt".to_string(),
        case_sensitive: false,
    };

    //panics if did not get an error
    run(config).expect_err("accessing an invalid file");
}

//the lifetime parameter `'a` specifies which argument lifetime is connected to
//the lifetime of the return value.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[test]
fn test_case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[test]
fn test_case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
        vec!["Rust:", "Trust me."],
        search_case_insensitive(query, contents)
    );
}
