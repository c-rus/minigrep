use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    //specific datatype annotation for collect to deduce the type
    let args: Vec<String> = env::args().collect();

    //borrow `args` for this function
    //handle result being returned by using a 'closure' (anonymous function)
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    //moves ownership of `config` into this function
    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

//use the trait object `Box<dyn Error>` to give flexibility in returning error values
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //`?` will return error value from current function for caller to handle
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    /// Returns a `Result` value that will contain a `Config` instance in the 
    /// successful case and will describe the problem in the error case.
    fn new(args: &[String]) -> Result<Config, &str> {
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