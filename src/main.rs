use std::env;
use std::process;

//bring `Config` type into scope from library crate
use minigrep::Config;

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
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}