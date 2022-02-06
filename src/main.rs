use std::env;
use std::fs;

fn main() {
    //specific datatype annotation for collect to deduce the type
    let args: Vec<String> = env::args().collect();

    //borrow `args` for this function
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    //reads the file
    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong with reading the file");
    
    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        //using panic! is more appropriate for a programming problem than a usage problem
        if args.len() < 3 {
            panic!("not enough arguments");
        }

        //the `args` variable is the owner of the argument values and is only
        //letting `parse_config` borrow them, which means we would violate Rust's
        //borrowing rules if `Config` tried to take ownership of the values in `args`.
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Config {
            query: query, 
            filename: filename,
        }
    }
}