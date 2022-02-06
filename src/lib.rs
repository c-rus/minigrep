use std::fs;
use std::error::Error;

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
            return Err("not enough arguments")
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

//use the trait object `Box<dyn Error>` to give flexibility in returning error values
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //`?` will return error value from current function for caller to handle
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}